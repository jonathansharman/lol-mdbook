use mdbook::{
	book::Book,
	errors::Error,
	preprocess::{Preprocessor, PreprocessorContext},
	BookItem,
};

#[derive(Default)]
pub struct Yell;

impl Preprocessor for Yell {
	fn name(&self) -> &str {
		"yell-preprocessor"
	}

	fn run(
		&self,
		_ctx: &PreprocessorContext,
		mut book: Book,
	) -> Result<Book, Error> {
		book.for_each_mut(|item| {
			if let BookItem::Chapter(ch) = item {
				ch.content = ch.content.to_uppercase();
			}
		});
		Ok(book)
	}

	fn supports_renderer(&self, renderer: &str) -> bool {
		renderer != "not-supported"
	}
}
