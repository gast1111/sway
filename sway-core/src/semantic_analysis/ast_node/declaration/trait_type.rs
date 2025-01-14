use sway_error::{
    error::CompileError,
    handler::{ErrorEmitted, Handler},
};
use sway_types::Span;

use crate::{
    language::{
        parsed,
        ty::{self, TyTraitType},
    },
    semantic_analysis::{
        type_check_context::EnforceTypeArguments, TypeCheckAnalysis, TypeCheckAnalysisContext,
        TypeCheckContext,
    },
    type_system::*,
    Engines,
};

impl ty::TyTraitType {
    pub(crate) fn type_check(
        handler: &Handler,
        mut ctx: TypeCheckContext,
        trait_type: parsed::TraitTypeDeclaration,
    ) -> Result<Self, ErrorEmitted> {
        let parsed::TraitTypeDeclaration {
            name,
            attributes,
            ty_opt,
            span,
        } = trait_type;

        let engines = ctx.engines();
        let type_engine = engines.te();

        let ty = if let Some(mut ty) = ty_opt {
            ty.type_id = ctx
                .resolve_type(
                    handler,
                    ty.type_id,
                    &ty.span,
                    EnforceTypeArguments::No,
                    None,
                )
                .unwrap_or_else(|err| type_engine.insert(engines, TypeInfo::ErrorRecovery(err)));
            Some(ty)
        } else {
            None
        };

        if let Some(implementing_type) = ctx.self_type() {
            Ok(ty::TyTraitType {
                name,
                attributes,
                ty,
                implementing_type,
                span,
            })
        } else {
            Err(handler.emit_err(CompileError::Internal("Self type not provided.", span)))
        }
    }

    /// Used to create a stubbed out constant when the constant fails to
    /// compile, preventing cascading namespace errors.
    pub(crate) fn error(engines: &Engines, decl: parsed::TraitTypeDeclaration) -> TyTraitType {
        let parsed::TraitTypeDeclaration {
            name,
            attributes,
            ty_opt,
            span,
        } = decl;
        TyTraitType {
            name,
            attributes,
            ty: ty_opt,
            implementing_type: engines
                .te()
                .insert(engines, TypeInfo::new_self_type(Span::dummy())),
            span,
        }
    }
}

impl TypeCheckAnalysis for ty::TyTraitType {
    fn type_check_analyze(
        &self,
        _handler: &Handler,
        _ctx: &mut TypeCheckAnalysisContext,
    ) -> Result<(), ErrorEmitted> {
        Ok(())
    }
}
