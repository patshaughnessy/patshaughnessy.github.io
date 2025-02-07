fswatch -o posts | xargs -n1 -I{} ./target/release/blogc -d posts .
