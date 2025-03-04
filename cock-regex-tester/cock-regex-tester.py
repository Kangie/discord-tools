#!/usr/bin/env python3
import unittest
import re

# This is kind-of useless; Discord uses the Rust `regex` crate which considers a match to a capture group
# to be a match!

# The regex for offensive words
offensive_regex = r"(c(ock|0ck|\*ck|kock)([-,\.])?(head|sucker|sucking|tease|block(ing)?|s)?)s?"

# The list of whitelisted words
whitelist = ["cockpit", "cockatoo", "cock-a-doodle-doo", "cockerel", "cocktail", "cocking", "cockings", "cockaded"]


# The function to check if a string is offensive
def is_offensive(text):
    # Extract words, this time treating hyphens as part of the word
    words = re.findall(r"\b[\w*-]+\b", text.lower())
    for word in words:
        is_whitelisted = False
        if word in whitelist:
            is_whitelisted = True
        if not is_whitelisted and re.search(offensive_regex, word):
            return True
    return False


# The test cases
class TestIsOffensive(unittest.TestCase):

    def test_offensive_words(self):
        self.assertTrue(is_offensive("cock"))
        self.assertTrue(is_offensive("cocksucker"))
        self.assertTrue(is_offensive("cockblocking"))
        self.assertTrue(is_offensive("c0ck"))

    def test_whitelisted_words(self):
        for word in whitelist:
            self.assertFalse(is_offensive(word))

    def test_capitalization(self):
        self.assertTrue(is_offensive("COCK"))
        self.assertTrue(is_offensive("cOCKsucking"))

    def test_hyphen_variations(self):
        self.assertTrue(is_offensive("cock-block"))
        self.assertTrue(is_offensive("cockblock"))

    def test_punctuation(self):
        self.assertTrue(is_offensive("cock,sucker"))
        self.assertTrue(is_offensive("cock.head"))
        self.assertTrue(is_offensive("c*ckhead!"))

    def test_pluralisation(self):
        self.assertTrue(is_offensive('cockheads'))
        self.assertTrue(is_offensive('cocks'))
        self.assertTrue(is_offensive('cocksuckers'))


if __name__ == '__main__':
    unittest.main()
