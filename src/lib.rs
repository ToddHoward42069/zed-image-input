use zed_extension_api as zed;

struct ImageInputExtension;

impl zed::Extension for ImageInputExtension {
    fn run_slash_command(
        &self,
        command: zed::SlashCommand,
        args: Vec<String>,
        _worktree: Option<&zed::Worktree>,
    ) -> Result<zed::SlashCommandOutput, String> {
        match command.name.as_str() {
            "image" => {
                if args.is_empty() {
                    return Err("Please provide an image file path.".to_string());
                }

                let image_path = &args[0];
                let image_data = std::fs::read(image_path)
                    .map_err(|e| format!("Failed to read image file: {}", e))?;

                let base64_image = base64::encode(&image_data);
                let markdown = format!("![Image](data:image/png;base64,{})", base64_image);

                Ok(zed::SlashCommandOutput {
                    sections: vec![zed::SlashCommandOutputSection {
                        range: (0..markdown.len()).into(),
                        label: "Image".to_string(),
                    }],
                    text: markdown,
                })
            }
            _ => Err(format!("Unknown slash command: {}", command.name)),
        }
    }

    fn complete_slash_command_argument(
        &self,
        command: zed::SlashCommand,
        _args: Vec<String>,
    ) -> Result<Vec<zed::SlashCommandArgumentCompletion>, String> {
        match command.name.as_str() {
            "image" => Ok(vec![zed::SlashCommandArgumentCompletion {
                label: "Enter image file path".to_string(),
                new_text: "".to_string(),
                run_command: false,
            }]),
            _ => Err(format!("Unknown slash command: {}", command.name)),
        }
    }
}

zed::register_extension!(ImageInputExtension);
// This implementation is already present in the file.
// The Extension trait and slash command functionality are already implemented.
// No additional code needs to be inserted here.
// The Extension trait and slash command functionality are already implemented.
// No additional code needs to be inserted here.
