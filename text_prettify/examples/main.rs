use text_prettify::graphics_code::{Graphics,prettified};

fn main() {
    let bold = Graphics::Bold;
    let italic = Graphics::Italic;
    let underline = Graphics::Underline;
    let strikethrough = Graphics::Strikethrough;

    let sentence = "Your sentence";

    let bold_text = prettified(sentence, bold.clone());
    let italic_text = prettified(sentence, italic.clone());
    let underline_text = prettified(sentence, underline.clone());
    let strikethrough_text = prettified(sentence, strikethrough.clone());

    println!("Bold Text:           {bold_text}");
    println!("Italic Text:         {italic_text}");
    println!("Underline Text:      {underline_text}");
    println!("Strikethrough Text:  {strikethrough_text}");
}
