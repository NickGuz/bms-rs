//! Definitions of the token in BMS format.

use std::path::Path;

use super::{command::*, cursor::Cursor, Result};

/// A token of BMS format.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token<'a> {
    /// `#ARTIST [string]`. Defines the artist name of the music.
    Artist(&'a str),
    /// `#@BGA[01-ZZ] [01-ZZ] [sx] [sy] [w] [h] [dx] [dy]`. Defines the image object from trimming the existing image object.
    AtBga {
        /// The id of the object to define.
        id: ObjId,
        /// The id of the object to be trimmed.
        source_bmp: ObjId,
        /// The top left point of the trim area in pixels.
        trim_top_left: (i16, i16),
        /// The size of the trim area in pixels.
        trim_size: (u16, u16),
        /// The top left point to be rendered in pixels.
        draw_point: (i16, i16),
    },
    /// `#BANNER [filename]`. Defines the banner image. This can be used on music select or result view. It should be 300x80.
    Banner(&'a Path),
    /// `#BACKBMP [filename]`. Defines the background image file of the play view. It should be 640x480. The effect will depend on the skin of the player.
    BackBmp(&'a Path),
    /// `#BGA[01-ZZ] [01-ZZ] [x1] [y1] [x2] [y2] [dx] [dy]`. Defines the image object from trimming the existing image object.
    Bga {
        /// The id of the object to define.
        id: ObjId,
        /// The id of the object to be trimmed.
        source_bmp: ObjId,
        /// The top left point of the trim area in pixels.
        trim_top_left: (i16, i16),
        /// The bottom right point of the trim area in pixels.
        trim_bottom_right: (i16, i16),
        /// The top left point to be rendered in pixels.
        draw_point: (i16, i16),
    },
    /// `#BMP[01-ZZ] [filename]`. Defines the background image/movie object. The file specified may be not only BMP format, and also PNG, AVI, MP4, MKV and others. Its size should be less than or equal to 256x256. The black (`#000000`) pixel in the image will be treated as transparent. When the id `00` is specified, this first field will be `None` and the image will be shown when the player get mistaken.
    Bmp(Option<ObjId>, &'a Path),
    /// `#BPM [f64]`. Defines the base Beats-Per-Minute of the score. Defaults to 130, but some players don't conform to it.
    Bpm(&'a str),
    /// `#BPM[01-ZZ] [f64]`. Defines the Beats-Per-Minute change object.
    BpmChange(ObjId, &'a str),
    /// `#CASE [u32]`. Starts a case scope if the integer equals to the generated random number. If there's no `#SKIP` command in the scope, the parsing will **fallthrough** to the next `#CASE` or `#DEF`. See also [`Token::Switch`].
    Case(u32),
    /// `#CHANGEOPTION[01-ZZ] [string]`. Defines the play option change object. Some players interpret and apply the preferences.
    ChangeOption(ObjId, &'a str),
    /// `#COMMENT [string]`. Defines the text which is shown in the music select view. This may or may not be surrounded by double-quotes.
    Comment(&'a str),
    /// `#DEF`. Starts a case scope if any `#CASE` had not matched to the generated random number. It must be placed in the end of the switch scope. See also [`Token::Switch`].
    Def,
    /// `#DIFFICULTY [1-5]`. Defines the difficulty of the score. It can be used to sort the score having the same title.
    Difficulty(u8),
    /// `#ELSEIF [u32]`. Starts an if scope when the preceding `#IF` had not matched to the generated random number. It must be in an if scope.
    Else,
    /// `#ELSEIF [u32]`. Starts an if scope when the integer equals to the generated random number. It must be in an if scope. If preceding `#IF` had matched to the generated, this scope don't start. Syntax sugar for:
    ///
    /// ```text
    /// #ELSE
    ///   #IF n
    ///   // ...
    ///   #ENDIF
    /// #ENDIF
    /// ```
    ElseIf(u32),
    /// `%EMAIL [string]`. The email address of this score file author.
    Email(&'a str),
    /// `#ENDIF`. Closes the if scope. See [Token::If].
    EndIf,
    /// `#ENDRANDOM`. Closes the random scope. See [Token::Random].
    EndRandom,
    /// `#ENDSWITCH`. Closes the random scope. See [Token::Switch].
    EndSwitch,
    /// `#BMP[01-ZZ] [0-255],[0-255],[0-255],[0-255] [filename]`. Defines the background image/movie object with the color (alpha, red, green and blue) which will be treated as transparent.
    ExBmp(ObjId, Argb, &'a Path),
    /// `#EXRANK[01-ZZ] [0-3]`. Defines the judgement level change object.
    ExRank(ObjId, JudgeLevel),
    /// `#EXWAV[01-ZZ] [parameter order] [pan or volume or frequency; 1-3] [filename]`. Defines the key sound object with the effect of pan, volume and frequency.
    ExWav(ObjId, [&'a str; 4], &'a Path),
    /// `#GENRE [string]`. Defines the genre of the music.
    Genre(&'a str),
    /// `#IF [u32]`. Starts an if scope when the integer equals to the generated random number. This must be placed in a random scope. See also [`Token::Random`].
    If(u32),
    /// `#LNOBJ [01-ZZ]`. Declares the object as the end of an LN. The preceding object of the declared will be treated as the beginning of an LN.
    LnObj(ObjId),
    /// `#LNTYPE 1`. Declares the LN notation as the RDM type.
    LnTypeRdm,
    /// `#LNTYPE 2`. Declares the LN notation as the MGQ type.
    LnTypeMgq,
    /// `#MAKER [string]`. Defines the author name of the score.
    Maker(&'a str),
    /// `#XXXYY:ZZ...`. Defines the message which places the object onto the score. `XXX` is the track, `YY` is the channel, and `ZZ...` is the object id sequence.
    Message {
        /// The track, or measure, must start from 1. But some player may allow the 0 measure (i.e. Lunatic Rave 2).
        track: Track,
        /// The channel commonly expresses what the lane be arranged the note to.
        channel: Channel,
        /// The message to the channel.
        message: &'a str,
    },
    /// `#MIDIFILE [filename]`. Defines the MIDI file as the BGM. *Deprecated*
    MidiFile(&'a Path),
    /// `#OCT/FP`. Declares the score as the octave mode.
    OctFp,
    /// `#OPTION [string]`. Defines the play option of the score. Some players interpret and apply the preferences.
    Option(&'a str),
    /// `#PATH_WAV [string]`. Defines the root path of [`Token::Wav`] paths. This should be used only for tests.
    PathWav(&'a Path),
    /// `#PLAYER [1-4]`. Defines the play style of the score.
    Player(PlayerMode),
    /// `#PLAYLEVEL [integer]`. Defines the difficulty level of the score. This can be used on music select view.
    PlayLevel(u8),
    /// `#POORBGA [0-2]`. Defines the display mode of the POOR BGA.
    PoorBga(PoorMode),
    /// `#RANDOM [u32]`. Starts a random scope which can contain only `#IF`-`#ENDIF` scopes. The random scope must close with `#ENDRANDOM`. A random integer from 1 to the integer will be generated when parsing the score. Then if the integer of `#IF` equals to the random integer, the commands in an if scope will be parsed, otherwise all command in it will be ignored. Any command except `#IF` and `#ENDIF` must not be included in the scope, but some players allow it.
    Random(u32),
    /// `#RANK [0-3]`. Defines the judgement level.
    Rank(JudgeLevel),
    /// `#SETRANDOM [u32]`. Starts a random scope but the integer will be used as the generated random number. It should be used only for tests.
    SetRandom(u32),
    /// `#SETSWITCH [u32]`. Starts a switch scope but the integer will be used as the generated random number. It should be used only for tests.
    SetSwitch(u32),
    /// `#SKIP`. Escapes the current switch scope. It is often used in the end of every case scope.
    Skip,
    /// `#STAGEFILE [filename]`. Defines the splashscreen image. It should be 640x480.
    StageFile(&'a Path),
    /// `#STOP[01-ZZ] [0-4294967295]`. Defines the stop object. The scroll will stop the beats of the integer divided by 192. A beat length depends on the current BPM. If there are other objects on same time, the stop object must be evaluated at last.
    Stop(ObjId, u32),
    /// `#SUBARTIST [string]`. Defines the sub-artist name of the music.
    SubArtist(&'a str),
    /// `#SUBTITLE [string]`. Defines the subtitle of the music.
    SubTitle(&'a str),
    /// `#SWITCH [u32]`. Starts a switch scope which can contain only `#CASE` or `#DEF` scopes. The switch scope must close with `#ENDSW`. A random integer from 1 to the integer will be generated when parsing the score. Then if the integer of `#CASE` equals to the random integer, the commands in a case scope will be parsed, otherwise all command in it will be ignored. Any command except `#CASE` and `#DEF` must not be included in the scope, but some players allow it.
    Switch(u32),
    /// `#TEXT[01-ZZ] string`. Defines the text object.
    Text(ObjId, &'a str),
    /// `#TITLE [string]`. Defines the title of the music.
    Title(&'a str),
    /// `#TOTAL [f64]`. Defines the total gauge percentage when all notes is got as PERFECT.
    Total(&'a str),
    /// `%URL [string]`. The url of this score file.
    Url(&'a str),
    /// `#VIDEOFILE [filename]` / `#MOVIE [filename]`. Defines the background movie file. The audio track in the movie file should not be played. The play should start from the track `000`.
    VideoFile(&'a Path),
    /// `#VOLWAV [0-255]`. Defines the relative volume percentage of the sound in the score.
    VolWav(Volume),
    /// `#WAV[01-ZZ] [filename]`. Defines the key sound object. When same id multiple objects ring at same time, it must be played only one. The file specified may be not only WAV format, and also OGG, MP3 and others.
    Wav(ObjId, &'a Path),
}

impl<'a> Token<'a> {
    pub(crate) fn parse(c: &mut Cursor<'a>) -> Result<Self> {
        loop {
            let command = c
                .next_token()
                .ok_or_else(|| c.err_expected_token("command"))?;

            break Ok(match command.to_uppercase().as_str() {
                "#PLAYER" => Self::Player(PlayerMode::from(c)?),
                "#GENRE" => Self::Genre(c.next_line_remaining()),
                "#TITLE" => Self::Title(c.next_line_remaining()),
                "#SUBTITLE" => Self::SubTitle(c.next_line_remaining()),
                "#ARTIST" => Self::Artist(c.next_line_remaining()),
                "#SUBARTIST" => Self::SubArtist(c.next_line_remaining()),
                "#DIFFICULTY" => Self::Difficulty(
                    c.next_token()
                        .ok_or_else(|| c.err_expected_token("difficulty"))?
                        .parse()
                        .map_err(|_| c.err_expected_token("integer"))?,
                ),
                "#STAEGFILE" => Self::StageFile(
                    c.next_token()
                        .map(Path::new)
                        .ok_or_else(|| c.err_expected_token("stage filename"))?,
                ),
                "#BANNER" => Self::Banner(
                    c.next_token()
                        .map(Path::new)
                        .ok_or_else(|| c.err_expected_token("banner filename"))?,
                ),
                "#BACKBMP" => Self::BackBmp(
                    c.next_token()
                        .map(Path::new)
                        .ok_or_else(|| c.err_expected_token("backbmp filename"))?,
                ),
                "#TOTAL" => Self::Total(
                    c.next_token()
                        .ok_or_else(|| c.err_expected_token("gauge increase rate"))?,
                ),
                "#BPM" => Self::Bpm(c.next_token().ok_or_else(|| c.err_expected_token("bpm"))?),
                "#PLAYLEVEL" => Self::PlayLevel(
                    c.next_token()
                        .ok_or_else(|| c.err_expected_token("play level"))?
                        .parse()
                        .map_err(|_| c.err_expected_token("integer"))?,
                ),
                "#RANK" => Self::Rank(JudgeLevel::from(c)?),
                "#LNTYPE" => {
                    if c.next_token() == Some("2") {
                        Self::LnTypeMgq
                    } else {
                        Self::LnTypeRdm
                    }
                }
                "#RANDOM" => {
                    let rand_max = c
                        .next_token()
                        .ok_or_else(|| c.err_expected_token("random max"))?
                        .parse()
                        .map_err(|_| c.err_expected_token("integer"))?;
                    Self::Random(rand_max)
                }
                "#ENDRANDOM" => Self::EndRandom,
                "#IF" => {
                    let rand_target = c
                        .next_token()
                        .ok_or_else(|| c.err_expected_token("random target"))?
                        .parse()
                        .map_err(|_| c.err_expected_token("integer"))?;
                    Self::If(rand_target)
                }
                "#ENDIF" => Self::EndIf,
                "#STAGEFILE" => {
                    if c.peek_token()
                        .expect("No token after STAGEFILE")
                        .starts_with("#")
                    {
                        // In my experience, there is often no value for this key,
                        // so just insert blank path for those cases
                        Self::StageFile(Path::new(""))
                    } else {
                        Self::StageFile(
                            c.next_token().map(Path::new).ok_or_else(|| {
                                c.err_expected_token("splashscreen imege filename")
                            })?,
                        )
                    }
                }
                "#VOLWAV" => {
                    let volume = c
                        .next_token()
                        .ok_or_else(|| c.err_expected_token("volume"))?
                        .parse()
                        .map_err(|_| c.err_expected_token("integer"))?;
                    Self::VolWav(Volume {
                        relative_percent: volume,
                    })
                }
                wav if wav.starts_with("#WAV") => {
                    let id = command.trim_start_matches("#WAV");
                    let str = c.next_line_remaining();
                    if str.is_empty() {
                        return Err(c.err_expected_token("key audio filename"));
                    }
                    let filename = Path::new(str);
                    Self::Wav(ObjId::from(id, c)?, filename)
                }
                bmp if bmp.starts_with("#BMP") => {
                    let id = command.trim_start_matches("#BMP");
                    let str = c.next_line_remaining();
                    if str.is_empty() {
                        return Err(c.err_expected_token("key audio filename"));
                    }
                    let filename = Path::new(str);
                    if id == "00" {
                        Self::Bmp(None, filename)
                    } else {
                        Self::Bmp(Some(ObjId::from(id, c)?), filename)
                    }
                }
                bpm if bpm.starts_with("#BPM") => {
                    let id = command.trim_start_matches("#BPM");
                    let bpm = c.next_token().ok_or_else(|| c.err_expected_token("bpm"))?;
                    Self::BpmChange(ObjId::from(id, c)?, bpm)
                }
                stop if stop.starts_with("#STOP") => {
                    let id = command.trim_start_matches("#STOP");
                    let stop = c
                        .next_token()
                        .ok_or_else(|| c.err_expected_token("stop beats"))?
                        .parse()
                        .map_err(|_| c.err_expected_token("integer"))?;
                    Self::Stop(ObjId::from(id, c)?, stop)
                }
                message
                    if message.starts_with('#')
                        && message.chars().nth(6) == Some(':')
                        && 8 <= message.len() =>
                {
                    let track = command[1..4]
                        .parse()
                        .map_err(|_| c.err_expected_token("[000-999]"))?;
                    let channel = &command[4..6];

                    let message = &command[7..];
                    Self::Message {
                        track: Track(track),
                        channel: Channel::from(channel, c)?,
                        message,
                    }
                }
                comment if !comment.starts_with('#') => {
                    c.next_line_remaining();
                    continue;
                }
                unknown => {
                    eprintln!("unknown command found: {:?}", unknown);
                    todo!();
                }
            });
        }
    }
}

/// A sequence of the [`Token`]. It can be used to run [`crate::parse::Bms::from_token_stream`].
pub struct TokenStream<'a> {
    tokens: Vec<Token<'a>>,
}

impl<'a> TokenStream<'a> {
    pub(crate) fn from_tokens(tokens: Vec<Token<'a>>) -> Self {
        Self { tokens }
    }

    /// Returns the borrowed iterator of the tokens.
    pub fn iter(&self) -> TokenStreamIter<'_, 'a> {
        TokenStreamIter {
            iter: self.tokens.iter(),
        }
    }
}

impl<'a> IntoIterator for TokenStream<'a> {
    type Item = Token<'a>;
    type IntoIter = <Vec<Token<'a>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}

/// An iterator of the [`TokenStream`].
#[derive(Debug)]
pub struct TokenStreamIter<'t, 'a> {
    iter: std::slice::Iter<'t, Token<'a>>,
}

impl<'t, 'a> Iterator for TokenStreamIter<'t, 'a> {
    type Item = &'t Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
