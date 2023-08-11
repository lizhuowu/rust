//! Validity checking for weak lang items

use rustc_data_structures::fx::FxHashSet;
use rustc_hir::lang_items::{self, LangItem};
use rustc_hir::weak_lang_items::WEAK_LANG_ITEMS;
use rustc_middle::middle::lang_items::required;
use rustc_middle::ty::TyCtxt;
use rustc_session::config::CrateType;

use crate::errors::{MissingLangItem, MissingPanicHandler, UnknownExternLangItem};

/// Checks the crate for usage of weak lang items, returning a vector of all the
/// language items required by this crate, but not defined yet.
pub fn check_crate(tcx: TyCtxt<'_>, items: &mut lang_items::LanguageItems) {
    // These are never called by user code, they're generated by the compiler.
    // They will never implicitly be added to the `missing` array unless we do
    // so here.
    if items.eh_personality().is_none() {
        items.missing.push(LangItem::EhPersonality);
    }
    if tcx.sess.target.os == "emscripten" && items.eh_catch_typeinfo().is_none() {
        items.missing.push(LangItem::EhCatchTypeinfo);
    }

    let crate_items = tcx.hir_crate_items(());
    for id in crate_items.foreign_items() {
        let attrs = tcx.hir().attrs(id.hir_id());
        if let Some((lang_item, _)) = lang_items::extract(attrs) {
            if let Some(item) = LangItem::from_name(lang_item) && item.is_weak() {
                if items.get(item).is_none() {
                    items.missing.push(item);
                }
            } else {
                let span = tcx.def_span(id.owner_id);
                tcx.sess.emit_err(UnknownExternLangItem { span, lang_item });
            }
        }
    }

    verify(tcx, items);
}

fn verify(tcx: TyCtxt<'_>, items: &lang_items::LanguageItems) {
    // We only need to check for the presence of weak lang items if we're
    // emitting something that's not an rlib.
    let needs_check = tcx.crate_types().iter().any(|kind| match *kind {
        CrateType::Dylib
        | CrateType::ProcMacro
        | CrateType::Cdylib
        | CrateType::Executable
        | CrateType::Staticlib => true,
        CrateType::Rlib => false,
    });
    if !needs_check {
        return;
    }

    let mut missing = FxHashSet::default();
    for &cnum in tcx.crates(()).iter() {
        for &item in tcx.missing_lang_items(cnum).iter() {
            missing.insert(item);
        }
    }

    for &item in WEAK_LANG_ITEMS.iter() {
        if missing.contains(&item) && required(tcx, item) && items.get(item).is_none() {
            if item == LangItem::PanicImpl {
                tcx.sess.emit_err(MissingPanicHandler);
            } else {
                tcx.sess.emit_err(MissingLangItem { name: item.name() });
            }
        }
    }
}
