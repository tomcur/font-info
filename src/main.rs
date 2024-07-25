use std::{
    io::{IsTerminal, Read, Write},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context};
use clap::Parser;
use serde::{ser::SerializeSeq, Serializer};
use swash::{FontDataRef, FontRef};

mod logger;

#[derive(Copy, Clone, clap::ValueEnum, Debug)]
enum OutputFormat {
    HumanReadable,
    Json,
}

/// Print metrics of fonts in a font file.
#[derive(Parser)]
struct Cli {
    /// Print verbose debug output.
    #[arg(short, long)]
    verbose: bool,

    /// Find all fonts in the giving file.
    #[arg(long, group = "input")]
    font_file: Option<PathBuf>,

    /// Find all fonts belonging to a font family using system font loading utilities.
    #[arg(long, group = "input")]
    family_name: Option<String>,

    /// The format of the output.
    #[arg(long, default_value = "human-readable")]
    format: OutputFormat,

    /// Print font features.
    #[arg(long)]
    print_features: bool,

    /// Print supported writing systems.
    #[arg(long)]
    print_writing_systems: bool,
}

enum PrintFeatures {
    Yes,
    No,
}

enum PrintWritingSystems {
    Yes,
    No,
}

struct Options {
    print_features: PrintFeatures,
    print_writing_systems: PrintWritingSystems,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Metrics {
    glyph_count: u16,
    units_per_em: u16,
    average_advance: f32,
    ascent: f32,
    descent: f32,
    leading: f32,
    line_height: f32,
    cap_height: f32,
    x_height: f32,
    stroke_size: f32,
    underline_offset: f32,
    strikeout_offset: f32,
}

impl Metrics {
    fn from_font(font: FontRef<'_>) -> Self {
        let swash::Metrics {
            glyph_count,
            units_per_em,
            average_width: average_advance,
            ascent,
            descent,
            leading,
            cap_height,
            x_height,
            stroke_size,
            underline_offset,
            strikeout_offset,
            ..
        } = font.metrics(&[]);
        Metrics {
            glyph_count,
            units_per_em,
            average_advance,
            ascent,
            descent,
            leading,
            line_height: ascent + descent,
            cap_height,
            x_height,
            stroke_size,
            underline_offset,
            strikeout_offset,
        }
    }
}

trait Out {
    fn push_font(
        &mut self,
        source: &str,
        font_index: usize,
        font: FontRef<'_>,
    ) -> anyhow::Result<()>;
}

impl<T: Out> Out for &mut T {
    fn push_font(
        &mut self,
        source: &str,
        font_index: usize,
        font: FontRef<'_>,
    ) -> anyhow::Result<()> {
        (*self).push_font(source, font_index, font)
    }
}

struct HumanReadable<W> {
    write: W,
    /// the number of fonts already written
    fonts_written: usize,
    /// output options
    options: Options,
}

impl<W> HumanReadable<W> {
    fn new(write: W, options: Options) -> Self {
        HumanReadable {
            write,
            fonts_written: 0,
            options,
        }
    }
}

impl<W: std::io::Write> Out for HumanReadable<W> {
    fn push_font(
        &mut self,
        source: &str,
        font_index: usize,
        font: FontRef<'_>,
    ) -> anyhow::Result<()> {
        let font_num = self.fonts_written + 1;
        write!(self.write, "-[ FONT {} ]-", font_num)?;

        {
            let char_width_idx = font_num.ilog10() + 1;
            for _ in 0..60 - 11 - char_width_idx {
                write!(self.write, "-")?;
            }
            writeln!(self.write)?;
        }

        writeln!(self.write, "              Source: {source}")?;
        writeln!(self.write, "Font index in source: {font_index}")?;

        {
            let attributes = font.attributes();
            let weight = attributes.weight().0;
            let style = attributes.style();
            let stretch = attributes.stretch().to_percentage() / 100.;

            writeln!(self.write, "              Weight: {weight}")?;
            writeln!(self.write, "               Style: {style}")?;
            writeln!(self.write, "             Stretch: {stretch:.2}")?;
        }

        if matches!(self.options.print_features, PrintFeatures::Yes) {
            write!(self.write, "            Features: ")?;
            for (idx, feat) in font.features().enumerate() {
                if idx > 0 {
                    write!(self.write, "\n                      ")?;
                }
                if let Some(name) = feat.name() {
                    write!(self.write, "{}:{:?}", name, feat.action())?;
                } else {
                    write!(self.write, "{:#X}:{:?}", feat.tag(), feat.action())?;
                }
            }
            writeln!(self.write)?;
        }

        if matches!(self.options.print_writing_systems, PrintWritingSystems::Yes) {
            write!(self.write, "     Writing systems: ")?;
            for (idx, writing_system) in font.writing_systems().enumerate() {
                if idx > 0 {
                    write!(self.write, "\n                      ")?;
                }
                if let Some(script) = writing_system.script() {
                    write!(self.write, "{script:?}")?;
                } else {
                    let tag = writing_system.script_tag();
                    if tag == 0x44464c54 {
                        write!(self.write, "Default")?;
                    } else {
                        write!(self.write, "{tag:#X}")?;
                    }
                }
                write!(self.write, ":")?;
                if let Some(language) = writing_system.language() {
                    if let Some(name) = language.name() {
                        write!(self.write, "{name}")?;
                    } else {
                        write!(self.write, "{}", language.language())?;
                    }
                } else {
                    let tag = writing_system.language_tag();
                    if tag == 0x44464c54 {
                        write!(self.write, "Default")?;
                    } else {
                        write!(self.write, "{tag:#X}")?;
                    }
                }
            }
            writeln!(self.write)?;
        }

        let Metrics {
            glyph_count,
            units_per_em,
            average_advance,
            ascent,
            descent,
            line_height,
            leading,
            cap_height,
            x_height,
            stroke_size,
            underline_offset,
            strikeout_offset,
        } = Metrics::from_font(font);

        writeln!(self.write, "         Glyph count: {glyph_count}")?;
        writeln!(self.write, "        Units per em: {units_per_em}")?;
        writeln!(self.write, "     Average advance: {average_advance}")?;
        writeln!(self.write, "              Ascent: {ascent}")?;
        writeln!(self.write, "             Descent: {descent}")?;
        writeln!(self.write, "         Line height: {line_height}")?;
        writeln!(self.write, "             Leading: {leading}")?;
        writeln!(self.write, "      Capital height: {cap_height}")?;
        writeln!(self.write, "          \"x\" height: {x_height}")?;
        writeln!(self.write, "    Stroke thickness: {stroke_size}")?;
        writeln!(self.write, "    Underline offset: {underline_offset}")?;
        writeln!(self.write, "    Strikeout offset: {strikeout_offset}")?;

        self.fonts_written += 1;

        Ok(())
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Feature {
    feature: u32,
    action: &'static str,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct WritingSystem {
    script: u32,
    language: u32,
}

struct Json<W> {
    serializer: W,
    options: Options,

    // reusable vector allocations
    features: Vec<Feature>,
    writing_systems: Vec<WritingSystem>,
}

impl<W: serde::ser::SerializeSeq> Json<W> {
    pub fn new(serializer: W, options: Options) -> Self {
        Json {
            serializer,
            options,

            features: Vec::new(),
            writing_systems: Vec::new(),
        }
    }
}

impl<W: serde::ser::SerializeSeq<Error = serde_json::Error>> Out for Json<W> {
    fn push_font(
        &mut self,
        source: &str,
        font_index: usize,
        font: FontRef<'_>,
    ) -> anyhow::Result<()> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Out<'m> {
            source: &'m str,
            font_index: usize,
            #[serde(skip_serializing_if = "Option::is_none")]
            features: Option<&'m [Feature]>,
            #[serde(skip_serializing_if = "Option::is_none")]
            writing_systems: Option<&'m [WritingSystem]>,
            metrics: &'m Metrics,
        }

        self.serializer.serialize_element(&Out {
            source,
            font_index,
            features: matches!(self.options.print_features, PrintFeatures::Yes).then(|| {
                self.features.clear();
                for feature in font.features() {
                    let action = match feature.action() {
                        swash::Action::Attachment => "attachment",
                        swash::Action::Adjustment => "adjustment",
                        swash::Action::Substitution => "substition",
                    };
                    self.features.push(Feature {
                        feature: feature.tag(),
                        action,
                    });
                }
                &*self.features
            }),
            writing_systems: matches!(self.options.print_writing_systems, PrintWritingSystems::Yes)
                .then(|| {
                    self.writing_systems.clear();
                    for writing_system in font.writing_systems() {
                        self.writing_systems.push(WritingSystem {
                            script: writing_system.script_tag(),
                            language: writing_system.language_tag(),
                        });
                    }
                    &*self.writing_systems
                }),
            metrics: &Metrics::from_font(font),
        })?;

        Ok(())
    }
}

fn main_(cli: Cli, mut out: impl Out) -> anyhow::Result<()> {
    match (cli.font_file, cli.family_name) {
        (Some(font_file), None) => {
            let font_file_name = font_file.to_string_lossy();

            log::info!("Reading font file '{font_file_name}'");

            let data = std::fs::read(&font_file)?;
            let font_data = FontDataRef::new(&data)
                .ok_or_else(|| anyhow!("Failed to parse font file: '{font_file_name}'",))?;

            for (idx, font) in font_data.fonts().enumerate() {
                out.push_font(&font_file_name, idx, font)?;
            }
        }
        (None, Some(family_name)) => {
            log::info!("Querying for font family '{family_name}'");

            let font_collection = font_enumeration::Collection::new().unwrap();

            let mut font_files: Vec<&'_ Path> = Vec::new();
            for font in font_collection.by_family(&family_name) {
                if !font_files.contains(&font.path.as_ref()) {
                    font_files.push(font.path.as_ref());
                }
            }

            let mut data = Vec::new();
            for font_file in font_files {
                data.clear();

                let font_file_name = font_file.to_string_lossy();

                log::info!("Reading font file '{font_file_name}'");

                let mut file =
                    std::fs::File::open(font_file).with_context(|| "Failed opening font file")?;
                if let Ok(metadata) = file.metadata() {
                    data.reserve(metadata.len() as usize);
                }
                file.read_to_end(&mut data)
                    .with_context(|| "Failed reading font file")?;

                let font_data = FontDataRef::new(&data)
                    .ok_or_else(|| anyhow!("Failed to parse font file: '{font_file_name}'",))?;

                for (idx, font) in font_data.fonts().enumerate() {
                    out.push_font(&font_file_name, idx, font)?;
                }
            }
        }
        (None, None) => {
            // read from stdin, but only if it is not a tty
            let mut stdin = std::io::stdin().lock();

            if stdin.is_terminal() {
                anyhow::bail!("Command-line argument '--font-file' or '--family-name' must be given. If neither argument is given and stdin is not an interactive terminal, this program attempts to parse the data on stdin as a font file.");
            }

            log::info!("Reading font data from stdin.");

            let mut data = Vec::new();
            stdin.read_to_end(&mut data)?;

            let font_data = swash::FontDataRef::new(&data)
                .ok_or_else(|| anyhow!("Failed to parse font file",))?;

            for (font_idx, font) in font_data.fonts().enumerate() {
                out.push_font("stdin", font_idx, font)?;
            }
        }
        (Some(_), Some(_)) => {
            // cli arguments --font-file and --family-name are mutually exclusive
            unreachable!()
        }
    };

    Ok(())
}

fn main() -> anyhow::Result<()> {
    logger::StderrLogger::init().expect("Failed to initialize logger");

    let cli = Cli::parse();

    if cli.verbose {
        log::set_max_level(log::LevelFilter::Trace);
    }

    let options = Options {
        print_features: if cli.print_features {
            PrintFeatures::Yes
        } else {
            PrintFeatures::No
        },
        print_writing_systems: if cli.print_writing_systems {
            PrintWritingSystems::Yes
        } else {
            PrintWritingSystems::No
        },
    };

    let mut stdout = std::io::stdout().lock();

    match cli.format {
        OutputFormat::HumanReadable => {
            let mut out = HumanReadable::new(&mut stdout, options);
            main_(cli, &mut out)?;
        }
        OutputFormat::Json => {
            let mut serializer = serde_json::Serializer::new(&mut stdout);
            let serialize_seq = serializer.serialize_seq(None).expect("infallible");
            let mut out = Json::new(serialize_seq, options);

            main_(cli, &mut out)?;

            let Json {
                serializer: ser_seq,
                ..
            } = out;
            ser_seq.end()?;
            writeln!(stdout)?;
        }
    }

    Ok(())
}
