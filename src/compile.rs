// Compile the Page
//
// Main part of page process.
//
// - Build AST using comrak
// - Change relative path to absolute path.
//   - if media, possibly with domain (based on cfg or mode)
// - Add relative markdown link list (intra w/ slugs)
// - Call comrak to convert markdown to html
