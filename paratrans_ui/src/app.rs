
use std::collections::HashMap;
use para_cal::para_to_GPU_api;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:

    model_para: String,
    unit: String,
    datatype: String,

    memory:String,
    unit_memory:String,

    gpu_type:String,
    gpu_num:String,
    
    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f64,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
	    model_para: "6.9".to_string(),
	    unit: "Billion".to_string(),
	    unit_memory:"GB".to_string(),
	    gpu_type:"4090".to_string(),
	    memory:"".to_string(),
	    gpu_num:"".to_string(),
	    datatype:"".to_string(),
            value: 2.7,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Transfering Parameters To Memory");

            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.model_para);
		egui::ComboBox::from_label("parameters.")
		    .selected_text(&self.unit)
		    .show_ui(ui, |ui|{
			ui.style_mut().wrap=Some(false);
			ui.selectable_value(&mut self.unit,
					    "Billion".to_string(),
					    "Billion",
			);
			ui.selectable_value(&mut self.unit,
					    "Million".to_string(),
					    "Million",
			);
			ui.selectable_value(&mut self.unit,
					    "Trillion".to_string(),
					    "Trillion",
			);
			ui.selectable_value(&mut self.unit,
					    "Unit".to_string(),
					    "Unit",
			);
		    });
            });

	    ui.horizontal(|ui|{
		ui.label("Datatype");
		ui.radio_value(&mut self.datatype,
			       "f32".to_string(),
			       "f32");
		ui.radio_value(&mut self.datatype,
			       "f16".to_string(),
			       "f16");
	    });

            ui.separator();

	    if ui.button("Calculate!").clicked(){
		let amount= self.model_para.parse::<f64>()
		    .unwrap_or(0.0);
	    let is_f32=if self.datatype=="f32"{true} else {false};
	    
		let (memory, gpu_num) = para_to_GPU_api(amount,
				self.unit.as_str(),
				is_f32,
				self.unit_memory.as_str(),
				self.gpu_type.as_str());

		// *(self.memory)=memory.to_string();
		// *(self.gpu_num)=gpu_num.to_string();
		self.memory=memory.to_string();
		self.gpu_num=gpu_num.to_string();
	    }


            ui.horizontal(|ui| {
		ui.label("Memory Used: ");
                ui.text_edit_singleline(&mut self.memory);

		egui::ComboBox::from_id_source("Memory Unit")
		    .selected_text(&self.unit_memory)
		    .show_ui(ui, |ui|{
			ui.style_mut().wrap=Some(false);
			ui.selectable_value(&mut self.unit_memory,
					    "GB".to_string(),
					    "GB",
			);
			ui.selectable_value(&mut self.unit_memory,
					    "TB".to_string(),
					    "TB",
			);
			ui.selectable_value(&mut self.unit_memory,
					    "MB".to_string(),
					    "MB",
			);
			ui.selectable_value(&mut self.unit_memory,
					    "KB".to_string(),
					    "KB",
			);
			ui.selectable_value(&mut self.unit_memory,
					    "Byte".to_string(),
					    "Byte",
			);
		    });

            });

            ui.separator();

            ui.horizontal(|ui| {
		ui.label("GPU Types: ");
		egui::ComboBox::from_id_source("GPU Types")
		    .selected_text(&self.gpu_type)
		    .show_ui(ui, |ui|{
			ui.style_mut().wrap=Some(false);
			let gpu_type_ls=vec!["4090",
					     "3090",
					     "4080",
					     "V100",
					     "A100",
					     "H800",
			];
			for gpu_type in gpu_type_ls {
			    ui.selectable_value(&mut self.gpu_type,
					    gpu_type.to_string(),
					    gpu_type,);
			}
                });
            });

            ui.horizontal(|ui| {
		ui.label("Number of Requirements:");
		ui.label(self.gpu_num.as_str());
            });




	    ui.separator();





















        });
    }
}

