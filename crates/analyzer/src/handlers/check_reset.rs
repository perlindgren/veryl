use crate::analyzer_error::AnalyzerError;
use veryl_parser::veryl_grammar_trait::*;
use veryl_parser::veryl_walker::{Handler, HandlerPoint, VerylWalker};
use veryl_parser::{ParolError, Stringifier};

#[derive(Default)]
pub struct CheckReset<'a> {
    pub errors: Vec<AnalyzerError>,
    text: &'a str,
    point: HandlerPoint,
    in_always_ff: bool,
    in_if_reset: bool,
    if_reset_brace: usize,
    if_reset_exist: bool,
    all_lefthand_sides: Vec<HierarchicalIdentifier>,
    reset_lefthand_sides: Vec<HierarchicalIdentifier>,
}

impl<'a> CheckReset<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            ..Default::default()
        }
    }
}

impl<'a> Handler for CheckReset<'a> {
    fn set_point(&mut self, p: HandlerPoint) {
        self.point = p;
    }
}

impl<'a> VerylGrammarTrait for CheckReset<'a> {
    fn l_brace(&mut self, _arg: &LBrace) -> Result<(), ParolError> {
        if let HandlerPoint::Before = self.point {
            if self.in_if_reset {
                self.if_reset_brace += 1;
            }
        }
        Ok(())
    }

    fn r_brace(&mut self, _arg: &RBrace) -> Result<(), ParolError> {
        if let HandlerPoint::Before = self.point {
            if self.in_if_reset {
                self.if_reset_brace -= 1;
                if self.if_reset_brace == 0 {
                    self.in_if_reset = false;
                }
            }
        }
        Ok(())
    }

    fn assignment_statement(&mut self, arg: &AssignmentStatement) -> Result<(), ParolError> {
        if let HandlerPoint::Before = self.point {
            if self.in_always_ff {
                self.all_lefthand_sides
                    .push(*arg.hierarchical_identifier.clone());
                if self.in_if_reset {
                    self.reset_lefthand_sides
                        .push(*arg.hierarchical_identifier.clone());
                }
            }
        }
        Ok(())
    }

    fn if_reset(&mut self, _arg: &IfReset) -> Result<(), ParolError> {
        if let HandlerPoint::Before = self.point {
            self.if_reset_exist = true;
            self.in_if_reset = true;
        }
        Ok(())
    }

    fn always_ff_declaration(&mut self, arg: &AlwaysFfDeclaration) -> Result<(), ParolError> {
        match self.point {
            HandlerPoint::Before => {
                // Check first if_reset when reset signel exists
                let if_reset_required = if arg.always_ff_declaration_opt.is_some() {
                    if let Some(x) = arg.always_ff_declaration_list.first() {
                        !matches!(&*x.statement, Statement::IfResetStatement(_))
                    } else {
                        true
                    }
                } else {
                    false
                };
                if if_reset_required {
                    self.errors.push(AnalyzerError::missing_if_reset(
                        self.text,
                        &arg.always_ff.always_ff_token,
                    ));
                }

                self.in_always_ff = true;
            }
            HandlerPoint::After => {
                // Check reset signal when if_reset exists
                if self.if_reset_exist && arg.always_ff_declaration_opt.is_none() {
                    self.errors.push(AnalyzerError::missing_reset_signal(
                        self.text,
                        &arg.always_ff.always_ff_token,
                    ));
                }

                // Check lefthand side values which is not reset
                let mut reset_lefthand_sides = Vec::new();
                for x in &self.reset_lefthand_sides {
                    let mut stringifier = Stringifier::new();
                    stringifier.hierarchical_identifier(x);
                    reset_lefthand_sides.push(stringifier.as_str().to_string());
                }

                for x in &self.all_lefthand_sides {
                    let mut stringifier = Stringifier::new();
                    stringifier.hierarchical_identifier(x);
                    let name = stringifier.as_str().to_string();
                    if self.if_reset_exist && !reset_lefthand_sides.contains(&name) {
                        self.errors.push(AnalyzerError::missing_reset_statement(
                            &name,
                            self.text,
                            &x.identifier.identifier_token,
                        ));
                    }
                }

                self.all_lefthand_sides.clear();
                self.reset_lefthand_sides.clear();
                self.in_always_ff = false;
                self.if_reset_exist = false;
            }
        }
        Ok(())
    }
}
