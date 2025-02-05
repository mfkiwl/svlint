use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use regex::Regex;
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct StyleOperatorArithmetic {
    re_split: Option<Regex>,
    re_op: Option<Regex>,
    re_succ: Option<Regex>,
}

impl Rule for StyleOperatorArithmetic {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> RuleResult {
        /*
        re_split extracts operator from anything following it.
        re_op is used to selectively apply this rule to specific operators.
        re_succ matches what is allowed after the operator.
            - nothing, immediately followed by non-whitespace
            - newline
            - exactly 1space, then comment
            - exactly 1space, then nothing
        */
        if self.re_split.is_none() {
            self.re_split = Some(Regex::new(r"(?P<op>\S+)(?P<succ>(?s:.)*)").unwrap());
        }
        if self.re_op.is_none() {
            let operators =
                [ "\\+" // {{{
                , "-"
                , "\\*"
                , "/"
                , "%"
                , "\\*\\*"
                ].join("|"); // }}}

            self.re_op = Some(Regex::new(format!("^({})$", operators).as_str()).unwrap());
        }
        if self.re_succ.is_none() {
            self.re_succ = Some(Regex::new(r"^($|[\n\v\f\r]| /| $)").unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };

        match node {
            RefNode::BinaryOperator(x) => {
                let re_split = self.re_split.as_ref().unwrap();
                let re_op = self.re_op.as_ref().unwrap();
                let t = syntax_tree.get_str(*x).unwrap();
                let caps = re_split.captures(&t).unwrap();

                if re_op.is_match(&caps[1]) {
                    let re_succ = self.re_succ.as_ref().unwrap();

                    if re_succ.is_match(&caps[2]) {
                        RuleResult::Pass
                    } else {
                        RuleResult::Fail
                    }
                } else {
                    RuleResult::Pass
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("style_operator_arithmetic")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Follow operator with a symbol, identifier, newline, or exactly 1 space.")
    }

    fn reason(&self) -> String {
        String::from("Consistent use of whitespace enhances readability by reducing visual noise.")
    }
}
