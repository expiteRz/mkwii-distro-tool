use std::{collections::HashMap, path::{Path, PathBuf}};

use crate::{parser::{RawConfig, cup_holder::{RawTrack, RawVariant}}, slots::{CourseId, MusicId}, traits::UiUpdateExt};

use anyhow::anyhow;
use iced::{Alignment::Center, Length::Fill, widget::{column, combo_box, container, keyed_column, row, scrollable, text, text_input}};
use mkwii_distro_tool_bmg::MessageGroup;
use mkwii_distro_tool_ui::{self as ui, Child};

pub mod parser;
pub mod slots;
pub mod traits;

const DEFAULT_CUP_BMG_START_ID: u32 = 0x10000;
const DEFAULT_TRACK_BMG_START_ID: u32 = 0x20000;
const DEFAULT_AUTHOR_BMG_START_ID: u32 = 0x30000;

#[derive(Clone)]
pub enum ParentMessage {
    Settings(SettingsMessage),
    Track(usize, TrackMessage),
    Error(String),
}

#[derive(Debug, Default, Clone)]
pub struct Config {
    pub settings: Settings,
    pub tracks: Vec<[Track; 4]>,
    pub cup_selected: usize,
    pub cup_names: Vec<String>,
}

impl ui::Child<ParentMessage> for Config {
    fn view(&self) -> iced::Element<'_, ParentMessage> {
        if !self.tracks.is_empty() { container(self.view_tracks()).into() } else { container("Cup is empty").into() }
    }

    fn update(&mut self, message: ParentMessage) {
        match message {
            ParentMessage::Settings(_settings) => {}
            ParentMessage::Track(i, track) => self.tracks[self.cup_selected][i].update(track),
            _ => {}
        }
    }
}

impl Config {
    pub fn view_tracks(&self) -> iced::Element<'_, ParentMessage> {
        let per_cup: iced::Element<ParentMessage> = keyed_column(
            self.tracks[self.cup_selected]
                .iter()
                .enumerate()
                .map(|(i, track)| (i, track.view().map(move |v| ParentMessage::Track(i, v)))),
        )
        .spacing(16)
        .into();
        scrollable(column![text(self.cup_names[self.cup_selected].clone()).size(16), per_cup].spacing(10))
            .height(Fill)
            .into()
    }
}

impl TryFrom<RawConfig> for Config {
    type Error = anyhow::Error;

    fn try_from(data: RawConfig) -> Result<Self, Self::Error> {
        let raw_settings = data.info;
        let raw_tracks = data.cups;
        let raw_variants = raw_tracks.variants;
        let mut variant_pos_per_track = 0;
        let _texts = MessageGroup::read_from_bmg(data.texts)?;
        let texts = _texts.items;
        let Some(idx_track_name) = texts.iter().position(|x| x.id == DEFAULT_TRACK_BMG_START_ID) else {
            return Err(anyhow!("Could not find track names in Config.pul."));
        };
        let Some(idx_track_author) = texts.iter().position(|x| x.id == DEFAULT_AUTHOR_BMG_START_ID) else {
            return Err(anyhow!("Could not find track authors in Config.pul."));
        };
        let Some(idx_cup_name) = texts.iter().position(|x| x.id == DEFAULT_CUP_BMG_START_ID) else {
            return Err(anyhow!("Could not find cup names in Config.pul"));
        };

        let mut settings = Settings::default();
        {
            settings.probabilities = raw_settings.probabilities;
            settings.region = raw_settings.region;
            settings.repick_prevention = raw_settings.repick_prevention;
            settings.allow_trophies = raw_settings.allow_trophies;
            settings.allow_200cc = raw_settings.allow_200cc;
            settings.allow_umt = raw_settings.allow_umt;
            settings.allow_feather = raw_settings.allow_feather;
            settings.allow_mega_cloud = raw_settings.allow_mega_cloud;
            settings.online_vote_timer = raw_settings.online_vote_timer;
        }
        // let mut tracks: Vec<[Track; 4]> = vec![];
        // {
        //     for (idx, raw_track) in raw_tracks.tracks.into_iter().enumerate() {
        //         let mut track = Track::new();
        //         track.slot = raw_track.slot;
        //         track.music = raw_track.music;
        //         track.name = texts[idx_track_name + idx].text.clone();
        //         track.author = texts[idx_track_author + idx].text.clone();
        //         if raw_track.variant_count != 0 {
        //             let mut variants: Vec<Variant> = vec![];
        //             for v in raw_variants[variant_pos_per_track..raw_track.variant_count as usize].into_iter() {
        //                 variants.push(Variant::from(v.clone()));
        //             }
        //             variant_pos_per_track = raw_track.variant_count as usize;
        //             track.variants = variants;
        //         }
        //         tracks.push(track);
        //     }
        // }
        let mut idx: usize = 0;
        let mut process_track = |x: RawTrack| {
            let mut track = Track::new();
            track.slot = x.slot;
            track.music = x.music;
            track.name = texts[idx_track_name + idx].text.clone();
            track.author = texts[idx_track_author + idx].text.clone();
            if x.variant_count != 0 {
                let mut variants: Vec<Variant> = vec![];
                for v in raw_variants[variant_pos_per_track..x.variant_count as usize].into_iter() {
                    variants.push(Variant::from(v.clone()));
                }
                variant_pos_per_track = x.variant_count as usize;
                track.variants = variants;
            }
            idx += 1;
            track
        };
        let tracks: Vec<[Track; 4]> = raw_tracks
            .tracks
            .chunks_exact(4)
            .map(|x| {
                [
                    process_track(x[0].clone()),
                    process_track(x[1].clone()),
                    process_track(x[2].clone()),
                    process_track(x[3].clone()),
                ]
            })
            .collect();
        let mut cup_names: Vec<String> = vec![];
        {
            for cup_idx in 0..tracks.len() {
                cup_names.push(texts[idx_cup_name + cup_idx].text.clone());
            }
        }

        Ok(Self {
            settings,
            tracks,
            cup_names,
            cup_selected: 0,
        })
    }
}

impl Config {
    pub fn read_from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let raw = RawConfig::read_from_path(path)?;
        Self::try_from(raw)
    }
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub probabilities: [u32; 2],
    pub region: u32,
    pub repick_prevention: u32,
    pub allow_trophies: bool, // Enable if ghosts exists
    pub allow_200cc: bool,
    pub allow_umt: bool,
    pub allow_feather: bool,
    pub allow_mega_cloud: bool,
    pub online_vote_timer: u8,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            probabilities: [60u32, 30u32],
            region: 7,
            repick_prevention: 16,
            allow_trophies: false,
            allow_200cc: false,
            allow_umt: false,
            allow_feather: false,
            allow_mega_cloud: false,
            online_vote_timer: 30,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    ChangeOnlineProbability150(u32),
    ChangeOnlineProbabilityMirror(u32),
    ChangeRegion(u32),
    ChangeAmountRepickPrevent(u32),
    Toggle200cc(bool),
    ToggleUltraTurbo(bool),
    ToggleFeather(bool),
    ToggleMegaCloud(bool),
    ChangeVoteTimer(u8),
}

#[derive(Debug, Clone)]
pub struct Track {
    pub slot_state: combo_box::State<CourseId>,
    pub music_state: combo_box::State<MusicId>,
    pub slot: CourseId,
    pub music: MusicId,
    pub path: PathBuf,
    pub name: String,
    pub author: String,
    pub variants: Vec<Variant>,
    pub ghosts: HashMap<u32, PathBuf>, // id (if variants), path to ghost
}

impl Track {
    pub fn new() -> Self {
        Self {
            slot_state: combo_box::State::new(CourseId::ALL.to_vec()),
            music_state: combo_box::State::new(MusicId::ALL.to_vec()),
            slot: Default::default(),
            music: Default::default(),
            path: Default::default(),
            name: Default::default(),
            author: Default::default(),
            variants: Default::default(),
            ghosts: Default::default(),
        }
    }
}

impl ui::Child<TrackMessage> for Track {
    fn view(&self) -> iced::Element<'_, TrackMessage> {
        let name = row![
            "Name:",
            text_input("Enter the track name...", &self.name).on_input(TrackMessage::ChangeName)
        ]
        .spacing(10)
        .align_y(Center);
        let author = row![
            "Author:",
            text_input("Enter the track authors...", &self.author).on_input(TrackMessage::ChangeAuthor)
        ]
        .spacing(10)
        .align_y(Center);
        let slot_selector = row![
            "Slot:",
            combo_box(&self.slot_state, "Pick a track slot..", Some(&self.slot), TrackMessage::ChangeSlot)
        ]
        .spacing(10)
        .align_y(Center);
        let music_selector = row![
            "Music:",
            combo_box(&self.music_state, "Pick a music slot..", Some(&self.music), TrackMessage::ChangeMusic)
        ]
        .spacing(10)
        .align_y(Center);

        log::debug!("view per track loaded");

        column![name, author, slot_selector, music_selector].spacing(4).into()
    }

    fn update(&mut self, message: TrackMessage) {
        match message {
            TrackMessage::ChangeSlot(course_id) => self.slot = course_id,
            TrackMessage::ChangeMusic(music_id) => self.music = music_id,
            TrackMessage::ChangeFilepath(path) => self.path = path,
            TrackMessage::ChangeName(name) => self.name = name,
            TrackMessage::ChangeAuthor(author) => self.author = author,
            TrackMessage::ManageVariants(variants) => self.variants.update(variants),
            TrackMessage::ManageVariant(idx, variant) => self.variants[idx].update(variant),
        }
    }
}

impl UiUpdateExt<VariantListMessage> for Vec<Variant> {
    fn update(&mut self, message: VariantListMessage) {
        match message {
            VariantListMessage::Add(path) => self.push(Variant::new(path)),
            VariantListMessage::Move(before, after) => {
                let target = self.remove(before);
                self.insert(after - 1, target);
            }
            VariantListMessage::Remove(idx) => {
                self.remove(idx);
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Variant {
    pub slot: CourseId,
    pub music: MusicId,
    pub path: PathBuf,
    pub name: String,
    pub author: String,
}

impl Variant {
    pub fn new(path: PathBuf) -> Self {
        Self { path, ..Default::default() }
    }
}

impl ui::Child<PerVariantMessage> for Variant {
    fn view(&self) -> iced::Element<'_, PerVariantMessage> {
        column![].into()
    }

    fn update(&mut self, message: PerVariantMessage) {
        match message {
            PerVariantMessage::ChangeSlot(course_id) => self.slot = course_id,
            PerVariantMessage::ChangeMusic(music_id) => self.music = music_id,
            PerVariantMessage::ChangeFilepath(path) => self.path = path,
            PerVariantMessage::ChangeName(name) => self.name = name,
            PerVariantMessage::ChangeAuthor(author) => self.author = author,
        }
    }
}

impl From<RawVariant> for Variant {
    fn from(value: RawVariant) -> Self {
        Self {
            slot: value.slot,
            music: value.music,
            path: Default::default(),
            name: Default::default(),
            author: Default::default(),
        }
    }
}

#[derive(Clone)]
pub enum TrackMessage {
    ChangeSlot(CourseId),
    ChangeMusic(MusicId),
    ChangeFilepath(PathBuf),
    ChangeName(String),
    ChangeAuthor(String),
    ManageVariants(VariantListMessage),
    ManageVariant(usize, PerVariantMessage),
}

#[derive(Clone)]
pub enum VariantListMessage {
    Add(PathBuf),
    Move(usize, usize),
    Remove(usize),
}

#[derive(Clone)]
pub enum PerVariantMessage {
    ChangeSlot(CourseId),
    ChangeMusic(MusicId),
    ChangeFilepath(PathBuf),
    ChangeName(String),
    ChangeAuthor(String),
}
