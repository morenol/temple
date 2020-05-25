use super::utils::assert_render_template_eq;
use temple::error::Result;

#[test]
fn render_plain_singe_line() -> Result<()> {
    assert_render_template_eq("Hello, world!", "Hello, world!")
}

#[test]
fn render_plain_multiline() -> Result<()> {
    assert_render_template_eq(
        "Hello, world!
Hello, world!",
        "Hello, world!
Hello, world!",
    )
}

#[test]
fn render_multiline_with_comment() -> Result<()> {
    assert_render_template_eq(
        "Hello, world!
{#Comment to skip #}Hello, world!",
        "Hello, world!
Hello, world!",
    )
}

#[test]
fn render_comment_with_code_inside() -> Result<()> {
    assert_render_template_eq(
        "(Hello World
{#Comment to
            {{for}}
            {{endfor}}
skip #}
{#Comment to
             {%
 skip #}
from Parser!)",
        "(Hello World


from Parser!)",
    )
}

#[test]
fn render_raw_test() -> Result<()> {
    assert_render_template_eq(
        "{% raw %}
    This is a raw text {{ 2 + 2 }}
{% endraw %}",
        "
    This is a raw text {{ 2 + 2 }}
",
    )
}
