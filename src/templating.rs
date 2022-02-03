use std::fs;
use std::path::Path;

use color_eyre::eyre::{Context, Result};
use log::debug;
use serde::Deserialize;

use crate::ticket_abstraction::AbstractTicket;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Template {
    pub chapters: Vec<Section>,
    pub sections: Option<Vec<Section>>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Section {
    pub title: String,
    pub filter: Filter,
    pub sections: Option<Vec<Section>>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Filter {
    pub doc_type: Option<Vec<String>>,
    pub subsystem: Option<Vec<String>>,
    pub component: Option<Vec<String>>,
}

impl Section {
    fn render(&self, tickets: &[AbstractTicket]) -> String {
        let heading = format!("= {}", self.title);
        let matching_tickets = tickets.iter().filter(|t| self.matches_ticket(t));
        let release_notes: Vec<_> = matching_tickets.map(|t| t.release_note()).collect();
        format!("{}\n\n{}", heading, release_notes.join("\n\n"))
    }

    fn matches_ticket(&self, ticket: &AbstractTicket) -> bool {
        let matches_doc_type = self
            .filter
            .doc_type
            .as_ref()
            .map_or(true, |dt| dt.contains(ticket.doc_type.as_ref().unwrap()));
        let matches_subsystem = self
            .filter
            .subsystem
            .as_ref()
            // TODO: Also take into account additional subsystems.
            .map_or(true, |sst| sst.contains(&ticket.subsystems[0]));
        let matches_component = self
            .filter
            .component
            .as_ref()
            // TODO: Also take into account additional components.
            .map_or(true, |c| c.contains(&ticket.components[0]));
        matches_doc_type && matches_subsystem && matches_component
    }
}

pub fn parse(template_file: &Path) -> Result<Template> {
    let text = fs::read_to_string(template_file).context("Cannot read the template file.")?;
    let templates: Template =
        serde_yaml::from_str(&text).context("Cannot parse the template file.")?;
    debug!("{:#?}", templates);
    Ok(templates)
}

pub fn format_document(tickets: &[AbstractTicket], template: &Template) -> String {
    let chapters: Vec<_> = template
        .chapters
        .iter()
        .map(|t| t.render(tickets))
        .collect();
    debug!("Chapters: {:#?}", chapters);

    let document = chapters.join("\n\n");
    debug!("Document: {}", document);
    document
}