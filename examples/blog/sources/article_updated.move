// <autogenerated>
//   This file was generated by dddappp code generator.
//   Any changes made to this file manually will be lost next time the file is regenerated.
// </autogenerated>

module rooch_examples::article_updated {

    use moveos_std::object_id::ObjectID;
    use rooch_examples::article::{Self, ArticleUpdated};
    use std::string::String;

    public fun id(article_updated: &ArticleUpdated): ObjectID {
        article::article_updated_id(article_updated)
    }

    public fun title(article_updated: &ArticleUpdated): String {
        article::article_updated_title(article_updated)
    }

    public fun body(article_updated: &ArticleUpdated): String {
        article::article_updated_body(article_updated)
    }

}
