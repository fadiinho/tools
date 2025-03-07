use crate::prelude::*;
use rome_formatter::write;

use crate::parentheses::{ExpressionNode, NeedsParentheses};
use rome_js_syntax::{JsAnyExpression, JsThisExpressionFields};
use rome_js_syntax::{JsSyntaxNode, JsThisExpression};

#[derive(Debug, Clone, Default)]
pub struct FormatJsThisExpression;

impl FormatNodeRule<JsThisExpression> for FormatJsThisExpression {
    fn fmt_fields(&self, node: &JsThisExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsThisExpressionFields { this_token } = node.as_fields();

        write![f, [this_token.format()]]
    }

    fn needs_parentheses(&self, item: &JsThisExpression) -> bool {
        item.needs_parentheses()
    }
}
impl NeedsParentheses for JsThisExpression {
    #[inline(always)]
    fn needs_parentheses(&self) -> bool {
        false
    }
    #[inline(always)]
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}

impl ExpressionNode for JsThisExpression {
    #[inline]
    fn resolve(&self) -> JsAnyExpression {
        self.clone().into()
    }

    #[inline]
    fn into_resolved(self) -> JsAnyExpression {
        self.into()
    }
}
