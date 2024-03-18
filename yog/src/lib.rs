pub mod link;
pub mod color;

// rust marco like CSS {} retunn "<style>...</style>"
// rust marco like HTML {} retunn "<html>...</html>"
// rust marco like JS {} retunn "<script>...</script>"
// rust marco like SVG {} retunn "<svg>...</svg>"
// rust marco like XML {} retunn "<xml>...</xml>"

macro_rules! css {
    ($($css:tt)*) => {
        format!("<style>{}</style>", $($css)*)
    };
}


macro_rules! tag {
    ($tag:literal, $($content:tt)*) => {
        format!("<{}>{}</{}>", $tag, $($content)*, $tag)
    };
}
//rust marco to replace http://edfsdfsdf to to https
macro_rules! https {
    ($url:literal) => {
        $url.replace("http://", "https://")
    };
}

//c style typeof
macro_rules! type_of {
    ($t:ty) => {
        std::any::type_name::<$t>()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css() {
        assert_eq!(css!("body { color: red; }"), "<style>body { color: red; }</style>");
    }
    #[test]
    fn test_tag() {
        assert_eq!(tag!("div", "Hello, World!"), "<div>Hello, World!</div>");
    }
}
