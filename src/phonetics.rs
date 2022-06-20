const phonemeSet: [&str, 2] = [
  " ",
  "!",
  "'",
  "(",
  ")",
  ",",
  "-",
  ".",
  ":",
  ";",
  "?",
  "a",
  "b",
  "c",
  "d",
  "e",
  "f",
  "h",
  "i",
  "j",
  "k",
  "l",
  "m",
  "n",
  "o",
  "p",
  "q",
  "r",
  "s",
  "t",
  "u",
  "v",
  "w",
  "x",
  "y",
  "z",
  "æ",
  "ç",
  "ð",
  "ø",
  "ħ",
  "ŋ",
  "œ",
  "ǀ",
  "ǁ",
  "ǂ",
  "ǃ",
  "ɐ",
  "ɑ",
  "ɒ",
  "ɓ",
  "ɔ",
  "ɕ",
  "ɖ",
  "ɗ",
  "ɘ",
  "ə",
  "ɚ",
  "ɛ",
  "ɜ",
  "ɞ",
  "ɟ",
  "ɠ",
  "ɡ",
  "ɢ",
  "ɣ",
  "ɤ",
  "ɥ",
  "ɦ",
  "ɧ",
  "ɨ",
  "ɪ",
  "ɫ",
  "ɬ",
  "ɭ",
  "ɮ",
  "ɯ",
  "ɰ",
  "ɱ",
  "ɲ",
  "ɳ",
  "ɴ",
  "ɵ",
  "ɶ",
  "ɸ",
  "ɹ",
  "ɺ",
  "ɻ",
  "ɽ",
  "ɾ",
  "ʀ",
  "ʁ",
  "ʂ",
  "ʃ",
  "ʄ",
  "ʈ",
  "ʉ",
  "ʊ",
  "ʋ",
  "ʌ",
  "ʍ",
  "ʎ",
  "ʏ",
  "ʐ",
  "ʑ",
  "ʒ",
  "ʔ",
  "ʕ",
  "ʘ",
  "ʙ",
  "ʛ",
  "ʜ",
  "ʝ",
  "ʟ",
  "ʡ",
  "ʢ",
  "ˈ",
  "ˌ",
  "ː",
  "ˑ",
  "˞",
  "β",
  "θ",
  "χ",
  "ᵻ",
  "ⱱ"
];

// https://github.com/espeak-ng/espeak-ng/issues/694
fn removeAdditionalSeparators(str: &str) -> &str {
 str.replaceAll(/_+/g, '_').replaceAll(/_ /g, ' ')
}

fn removeLineBreaks(str: &str) { str.replaceAll('\n', ' ') }
fn removeExtraSpaces(str: &str)  {str.replaceAll('  ', ' ')}
fn removeNonPhoneticChars(str: &str) =>
  Array.from(str).filter(phonemeSet.has.bind(phonemeSet)).join('');

const sanitizeEspeakOutput = pipe(
  removeAdditionalSeparators,
  removeLineBreaks,
  removeExtraSpaces,
  removeNonPhoneticChars
);

// TODO: Handle pure whitespace
/** Adds the starting and ending whitespace from the source string to the target string */
const preserveBoundaryWhitespace = (source: string, target: string) => {
  const startingWhitespace = source.length - source.trimStart().length;
  const endingWhitespace = source.length - source.trimEnd().length;
  return (
    source.slice(0, startingWhitespace) +
    target.trim() +
    (endingWhitespace === 0 ? '' : source.slice(-endingWhitespace))
  );
};

const collapseWhitespace = (str: string) => str.replace(/\s*([!,-.:;? '()])\s*/, '$1');

const toPhonetics = (texts: string[]) =>
  textToPhonemes(texts).then((texts) => texts.map(sanitizeEspeakOutput));

export async function stringToPhonetics(
  texts: string[],
  { preservePunctuation = false } = {}
): Promise<string[]> {
  if (preservePunctuation) {
    const splitTexts = texts
      .map((text) => text.replaceAll(/([0-9]),([0-9])/g, '$1$2'))
      .map((text) => ({ originalText: text, ...extractPunctuation(text) }));
    const phonemizedTexts = await toPhonetics(splitTexts.flatMap(prop('texts')));
    let pos = 0;
    return splitTexts.map(({ originalText, texts, punctuations }) => {
      const phonemized = phonemizedTexts.slice(pos, pos + texts.length);
      pos += texts.length;
      const process = pipe(
        (phonemized) => restorePunctuation(punctuations, phonemized),
        (phonemized) => preserveBoundaryWhitespace(originalText, phonemized),
        collapseWhitespace,
        removeLineBreaks
      );
      return process(phonemized);
    });
  }
  return toPhonetics(texts);
}
