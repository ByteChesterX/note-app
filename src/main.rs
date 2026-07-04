mod note;
mod storage;

use eframe::egui;
use note::Note;
use std::path::PathBuf;
use storage::{load_notes, save_notes};

struct NoteApp {
    notes: Vec<Note>,
    selected_idx: Option<usize>,
    title: String,
    category: String,
    tags: String,
    content: String,
    search: String,
    filter_category: String,
    filter_tag: String,
    save_message: String,
}

impl Default for NoteApp {
    fn default() -> Self {
        let notes = load_notes();
        Self {
            notes,
            selected_idx: None,
            title: String::new(),
            category: "Genel".to_string(),
            tags: String::new(),
            content: String::new(),
            search: String::new(),
            filter_category: String::new(),
            filter_tag: String::new(),
            save_message: String::new(),
        }
    }
}

impl NoteApp {
    fn save_current(&mut self) {
        if self.title.trim().is_empty() {
            self.save_message = "Başlık boş!".to_string();
            return;
        }
        let cat = if self.category.trim().is_empty() {
            "Genel"
        } else {
            self.category.trim()
        };
        let tags: Vec<String> = self
            .tags
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if let Some(idx) = self.selected_idx {
            self.notes[idx].update(
                self.title.trim(),
                &self.content,
                cat,
                tags,
            );
        } else {
            let note = Note::new(self.title.trim(), &self.content, cat, tags);
            self.notes.push(note);
            self.selected_idx = Some(self.notes.len() - 1);
        }
        save_notes(&self.notes).ok();
        self.save_message = "Kaydedildi".to_string();
    }

    fn load_note(&mut self, idx: usize) {
        let note = &self.notes[idx];
        self.title = note.title.clone();
        self.category = note.category.clone();
        self.tags = note.tags.join(", ");
        self.content = note.content.clone();
        self.selected_idx = Some(idx);
        self.save_message.clear();
    }

    fn filtered_notes(&self) -> Vec<usize> {
        let q = self.search.to_lowercase();
        self.notes
            .iter()
            .enumerate()
            .filter(|(_, n)| {
                let matches_search = q.is_empty()
                    || n.title.to_lowercase().contains(&q)
                    || n.content.to_lowercase().contains(&q)
                    || n.tags.iter().any(|t| t.to_lowercase().contains(&q))
                    || n.category.to_lowercase().contains(&q);
                let matches_cat = self.filter_category.is_empty()
                    || n.category == self.filter_category;
                let matches_tag = self.filter_tag.is_empty()
                    || n.tags.contains(&self.filter_tag);
                matches_search && matches_cat && matches_tag
            })
            .map(|(i, _)| i)
            .collect()
    }
}

impl eframe::App for NoteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::light());

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("📝 Not Defteri");
                if ui.button("+ Yeni").clicked() {
                    self.selected_idx = None;
                    self.title.clear();
                    self.category = "Genel".to_string();
                    self.tags.clear();
                    self.content.clear();
                    self.save_message.clear();
                }
            });
        });

        egui::SidePanel::left("note_list")
            .resizable(true)
            .default_width(280.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.search)
                            .hint_text("🔍 Ara...")
                            .desired_width(f32::INFINITY),
                    );

                    ui.horizontal(|ui| {
                        ui.label("Kategori:");
                        let mut all_cats: Vec<&str> = self
                            .notes
                            .iter()
                            .map(|n| n.category.as_str())
                            .collect();
                        all_cats.sort();
                        all_cats.dedup();
                        let current = if self.filter_category.is_empty() {
                            "Tümü".to_string()
                        } else {
                            self.filter_category.clone()
                        };
                        egui::ComboBox::from_id_salt("cat_filter")
                            .selected_text(&current)
                            .show_ui(ui, |ui| {
                                if ui.selectable_label(
                                    self.filter_category.is_empty(),
                                    "Tümü",
                                )
                                .clicked()
                                {
                                    self.filter_category.clear();
                                }
                                for cat in &all_cats {
                                    if ui
                                        .selectable_label(
                                            self.filter_category == *cat,
                                            *cat,
                                        )
                                        .clicked()
                                    {
                                        self.filter_category = cat.to_string();
                                    }
                                }
                            });
                    });

                    ui.horizontal(|ui| {
                        ui.label("Etiket:");
                        let mut all_tags: Vec<&str> = self
                            .notes
                            .iter()
                            .flat_map(|n| n.tags.iter().map(|t| t.as_str()))
                            .collect();
                        all_tags.sort();
                        all_tags.dedup();
                        let current = if self.filter_tag.is_empty() {
                            "Tümü".to_string()
                        } else {
                            self.filter_tag.clone()
                        };
                        egui::ComboBox::from_id_salt("tag_filter")
                            .selected_text(&current)
                            .show_ui(ui, |ui| {
                                if ui.selectable_label(
                                    self.filter_tag.is_empty(),
                                    "Tümü",
                                )
                                .clicked()
                                {
                                    self.filter_tag.clear();
                                }
                                for tag in &all_tags {
                                    if ui
                                        .selectable_label(
                                            self.filter_tag == *tag,
                                            *tag,
                                        )
                                        .clicked()
                                    {
                                        self.filter_tag = tag.to_string();
                                    }
                                }
                            });
                    });

                    ui.separator();

                    let filtered = self.filtered_notes();
                    let selected_in_list = self
                        .selected_idx
                        .map(|i| filtered.iter().position(|&x| x == i))
                        .flatten();

                    egui::ScrollArea::vertical()
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            for (pos, &real_idx) in filtered.iter().enumerate() {
                                let note = &self.notes[real_idx];
                                let is_selected = selected_in_list == Some(pos);
                                let label = format!(
                                    "{} [{}]",
                                    note.title,
                                    note.category
                                );
                                if ui
                                    .selectable_label(is_selected, &label)
                                    .clicked()
                                {
                                    self.load_note(real_idx);
                                }
                            }
                        });
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.title)
                        .hint_text("Not başlığı")
                        .desired_width(f32::INFINITY)
                        .font(egui::TextStyle::Heading),
                );

                ui.horizontal(|ui| {
                    ui.label("Kategori:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.category)
                            .desired_width(200.0),
                    );
                    ui.label("  Etiketler:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.tags)
                            .hint_text("virgülle ayırın")
                            .desired_width(250.0),
                    );
                });

                ui.add_space(5.0);

                ui.add(
                    egui::TextEdit::multiline(&mut self.content)
                        .hint_text("Markdown ile notunuzu yazın...")
                        .desired_width(f32::INFINITY)
                        .desired_rows(20)
                        .font(egui::TextStyle::Monospace),
                );

                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    if ui
                        .button("💾 Kaydet")
                        .clicked()
                    {
                        self.save_current();
                    }
                    if self.selected_idx.is_some() {
                        if ui
                            .button("🗑 Sil")
                            .clicked()
                        {
                            if let Some(idx) = self.selected_idx {
                                self.notes.remove(idx);
                                self.selected_idx = None;
                                self.title.clear();
                                self.category = "Genel".to_string();
                                self.tags.clear();
                                self.content.clear();
                                save_notes(&self.notes).ok();
                                self.save_message = "Silindi".to_string();
                            }
                        }
                    }
                    if ui
                        .button("👁 Önizle")
                        .clicked()
                    {
                        if !self.content.trim().is_empty() {
                            let md = self.content.clone();
                            let parser = pulldown_cmark::Parser::new(&md);
                            let mut html = String::from(
                                r#"<!DOCTYPE html><html><head><meta charset="utf-8"><style>
body { font-family: sans-serif; max-width: 800px; margin: 40px auto; padding: 0 20px; line-height: 1.6; }
pre { background: #f4f4f4; padding: 10px; border-radius: 5px; overflow-x: auto; }
code { background: #f4f4f4; padding: 2px 5px; border-radius: 3px; }
img { max-width: 100%; }
blockquote { border-left: 3px solid #ccc; margin-left: 0; padding-left: 15px; color: #666; }
</style></head><body>"#,
                            );
                            pulldown_cmark::html::push_html(&mut html, parser);
                            html.push_str("</body></html>");
                            let path = std::env::temp_dir()
                                .join(format!("preview_{}.html", uuid::Uuid::new_v4()));
                            std::fs::write(&path, &html).ok();
                            open::that(&path).ok();
                        }
                    }
                });

                if !self.save_message.is_empty() {
                    ui.add_space(5.0);
                    ui.colored_label(
                        egui::Color32::GREEN,
                        &self.save_message,
                    );
                }

                ui.add_space(10.0);
                ui.separator();
                ui.label("📊 İstatistikler");
                let total = self.notes.len();
                let cats = self
                    .notes
                    .iter()
                    .map(|n| n.category.as_str())
                    .collect::<std::collections::HashSet<_>>()
                    .len();
                let tags = self
                    .notes
                    .iter()
                    .flat_map(|n| n.tags.iter().map(|t| t.as_str()))
                    .collect::<std::collections::HashSet<_>>()
                    .len();
                ui.label(format!("Toplam: {total} not, {cats} kategori, {tags} etiket"));
            });
        });
    }
}

fn ensure_desktop_file() {
    let desktop_path = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("applications")
        .join("note-app.desktop");

    let exec_path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("note-app"));
    let content = format!(
        "[Desktop Entry]\n\
         Name=Not Defteri\n\
         Comment=Kişisel Not Defteri\n\
         Exec={}\n\
         Icon=accessories-text-editor\n\
         Terminal=false\n\
         Type=Application\n\
         Categories=Utility;TextEditor;\n\
         StartupNotify=true\n",
        exec_path.display()
    );

    if let Some(parent) = desktop_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    std::fs::write(&desktop_path, &content).ok();
}

fn main() -> Result<(), eframe::Error> {
    ensure_desktop_file();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([950.0, 650.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Not Defteri",
        options,
        Box::new(|_cc| Ok(Box::new(NoteApp::default()))),
    )
}
