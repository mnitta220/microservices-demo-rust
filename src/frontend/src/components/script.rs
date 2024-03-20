pub fn write(buf: &mut String) -> Result<(), &'static str> {
    buf.push_str(
        r#"<script src="https://stackpath.bootstrapcdn.com/bootstrap/4.1.1/js/bootstrap.min.js" "#,
    );
    buf.push_str(
        r#"integrity="sha384-smHYKdLADwkXOn1EmN1qk/HfnUcbVRZyYmZ4qpPea6sjB/pTJ0euyQp0Mk8ck+5T" "#,
    );
    buf.push_str(r#"crossorigin="anonymous">"#);
    buf.push_str(r#"</script>"#);

    Ok(())
}
