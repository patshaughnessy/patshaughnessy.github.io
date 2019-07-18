extern crate regex;
extern crate chrono;

use post::Post;

#[derive(Debug, Clone, Eq)]
pub struct PostLink {
    pub date_string: Option<String>,
    pub url: String,
    pub title: String
}

impl PartialEq for PostLink {
    fn eq(&self, other: &Self) -> bool {
        self.date_string == other.date_string &&
        self.url == other.url &&
        self.title == other.title
    }
}

impl PostLink {
    pub fn all_from(posts: &Vec<Post>) -> Vec<PostLink> {
        let mut last_date_string: Option<String> = None;
        posts.iter().map(|p| {
            let date_string = p.date.format("%B %Y").to_string();
            let post_link = match last_date_string {
                Some(ref last) if last == &date_string =>
                    PostLink {
                        date_string: None,
                        url: p.url.clone(),
                        title: p.title.clone()
                    },
                _ => PostLink {
                        date_string: Some(date_string.clone()),
                        url: p.url.clone(),
                        title: p.title.clone()
                    }
            };
            last_date_string = Some(date_string);
            post_link
        }).collect()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::env;
    use std::path::PathBuf;

    #[test]
    fn it_returns_an_empty_array_of_links() {
        let no_posts = &[].to_vec();
        assert_eq!(PostLink::all_from(no_posts), [].to_vec());
    }

    #[test]
    fn it_returns_a_valid_array_of_links() {
        let post = Post::from(&input_path()).unwrap();
        let post2 = Post::from(&input_path2()).unwrap();
        let posts = [post, post2].to_vec();
        let links = PostLink::all_from(&posts);
        assert_eq!(links.len(), 2);
        let link = &links[0];
        assert_eq!(link.date_string, Some("January 2018".to_string()));
        let link = &links[1];
        assert_eq!(link.date_string, Some("April 2009".to_string()));
    }

    #[test]
    fn it_returns_links_without_repeated_months() {
        let post = Post::from(&input_path()).unwrap();
        let mut post2 = Post::from(&input_path2()).unwrap();
        post2.date = post.date;
        let posts = [post, post2].to_vec();
        let links = PostLink::all_from(&posts);
        assert_eq!(links.len(), 2);
        let link = &links[0];
        assert_eq!(link.date_string, Some("January 2018".to_string()));
        let link = &links[1];
        assert_eq!(link.date_string, None);
    }

    fn input_path() -> PathBuf {
        tests_path().join("2018-01-18-learning-rust-if-let-vs-match.markdown")
    }

    fn input_path2() -> PathBuf {
        tests_path().join("2009-04-14-database-storage-for-paperclip-rewritten-to-use-a-single-table.markdown")
    }

    fn tests_path() -> PathBuf {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("tests")
    }
}
