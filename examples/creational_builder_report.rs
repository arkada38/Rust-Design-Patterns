use design_patterns::creational::builder::report::{
    HtmlReportBuilder, MarkdownReportBuilder, ReportBuilder,
};

/// Example with *Builder* pattern
fn main() {
    let html_report = HtmlReportBuilder::new()
        .with_header("Solar Deities : Hindu Mythological Story")
        .with_paragraph(
            "Let us enjoy reading this Hindu Mythological Story \
            of Solar Deities.",
        )
        .with_paragraph(
            "Ravana once went to challenge Surya, the Sun-God, to a fight. \
            When he reached the Solar Region he saw that the sun was about to rise and sent an envoy \
            to inform Surya of his arrival and the reason for his coming.",
        )
        .finish();

    println!("{}", html_report);

    let markdown_report = MarkdownReportBuilder::new()
        .with_header("Why Snakes Have Forked Tongues")
        .with_paragraph(
            "Garuda brought the nectar after overcoming numerous obstacles, \
            battling even the gods in the process.",
        )
        .with_paragraph(
            "The nagas were delighted when he placed the pot \
            containing the nectar before them. \
            They let Vinata go and then they went to wash themselves before partaking of the ambrosia.",
        )
        .with_header("Second header")
        .with_paragraph("Third paragraph")
        .finish();

    println!("{}", markdown_report);
}
