use crate::{
    Token,
    char_string::char_string,
    patterns::{AnyCapitalization, OwnedPatternExt, Pattern, SequencePattern},
};

use super::{Lint, LintKind, PatternLinter, Suggestion};

pub struct Confident {
    pattern: Box<dyn Pattern>,
}

impl Default for Confident {
    fn default() -> Self {
        let pattern = SequencePattern::default()
            .then(
                (|tok: &Token, _source: &[char]| tok.kind.is_verb() || tok.kind.is_article())
                    .or(Box::new(AnyCapitalization::new(char_string!("very")))),
            )
            .then_whitespace()
            .t_aco("confidant");

        Self {
            pattern: Box::new(pattern),
        }
    }
}

impl PatternLinter for Confident {
    fn pattern(&self) -> &dyn Pattern {
        self.pattern.as_ref()
    }

    fn match_to_lint(&self, matched_tokens: &[Token], source: &[char]) -> Option<Lint> {
        let span = matched_tokens.last()?.span;

        Some(Lint {
            span,
            lint_kind: LintKind::WordChoice,
            suggestions: vec![Suggestion::ReplaceWith("confident".chars().collect())],
            message: "Use the adjective.".to_owned(),
            priority: 127,
        })
    }

    fn description(&self) -> &'static str {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::Confident;
    use crate::linting::tests::{assert_lint_count, assert_suggestion_result};

    #[test]
    fn describing_person_incorrect() {
        assert_suggestion_result(
            "She felt confidant about her presentation.",
            Confident::default(),
            "She felt confident about her presentation.",
        );
    }

    #[test]
    fn describing_person_correct() {
        assert_lint_count(
            "She felt confident about her presentation.",
            Confident::default(),
            0,
        );
    }

    #[test]
    fn certainty_incorrect() {
        assert_suggestion_result(
            "I am confidant the test results are accurate.",
            Confident::default(),
            "I am confident the test results are accurate.",
        );
    }

    #[test]
    fn certainty_correct() {
        assert_lint_count(
            "I am confident the test results are accurate.",
            Confident::default(),
            0,
        );
    }

    #[test]
    fn demeanor_incorrect() {
        assert_suggestion_result(
            "He walked to the stage with a confidant stride.",
            Confident::default(),
            "He walked to the stage with a confident stride.",
        );
    }

    #[test]
    fn demeanor_correct() {
        assert_lint_count(
            "He walked to the stage with a confident stride.",
            Confident::default(),
            0,
        );
    }

    #[test]
    fn professional_incorrect() {
        assert_suggestion_result(
            "You should sound confidant during job interviews.",
            Confident::default(),
            "You should sound confident during job interviews.",
        );
    }

    #[test]
    fn professional_correct() {
        assert_lint_count(
            "You should sound confident during job interviews.",
            Confident::default(),
            0,
        );
    }

    #[test]
    fn assured_tone_incorrect() {
        assert_suggestion_result(
            "Present your argument in a confidant, persuasive manner.",
            Confident::default(),
            "Present your argument in a confident, persuasive manner.",
        );
    }

    #[test]
    fn assured_tone_correct() {
        assert_lint_count(
            "Present your argument in a confident, persuasive manner.",
            Confident::default(),
            0,
        );
    }

    #[test]
    fn extra_text_between() {
        assert_suggestion_result(
            "She felt very confidant about her presentation.",
            Confident::default(),
            "She felt very confident about her presentation.",
        );
    }

    #[test]
    fn linking_verb_was_confidant() {
        assert_suggestion_result(
            "She was confidant about her presentation.",
            Confident::default(),
            "She was confident about her presentation.",
        );
    }
}
