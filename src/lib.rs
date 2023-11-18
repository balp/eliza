pub struct Eliza {}

impl Eliza {
    pub fn process(&self, _question: &str) -> String {
        "".to_string()
    }

    pub fn new(_script: &str) -> Eliza {
        Eliza {}
    }
}


#[cfg(test)]
mod acceptance_tests {
    use super::*;

    macro_rules! eliza_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
            #[test]
            fn $name() {
                let doctor_script_modified: &str =  r#"(HOW DO YOU DO.  PLEASE TELL ME YOUR PROBLEM)\n

        START

        (SORRY
            ((0)
                (PLEASE DON'T APOLIGIZE)
                (APOLOGIES ARE NOT NECESSARY)
                (WHAT FEELINGS DO YOU HAVE WHEN YOU APOLOGIZE)
                (I'VE TOLD YOU THAT APOLOGIES ARE NOT REQUIRED)))

        (DONT = DON'T)
        (CANT = CAN'T)
        (WONT = WON'T)

        (REMEMBER 5
            ((0 YOU REMEMBER 0)
                 (DO YOU OFTEN THINK OF 4)
                 (DOES THINKING OF 4 BRING ANYTHING ELSE TO MIND)
                 (WHAT ELSE DO YOU REMEMBER)
                 (WHY DO YOU REMEMBER 4 JUST NOW)
                 (WHAT IN THE PRESENT SITUATION REMINDS YOU OF 4)
                 (WHAT IS THE CONNECTION BETWEEN ME AND 4))
            ((0 DO I REMEMBER 0)
                 (DID YOU THINK I WOULD FORGET 5)
                 (WHY DO YOU THINK I SHOULD RECALL 5 NOW)
                 (WHAT ABOUT 5)
                 (=WHAT)
                 (YOU MENTIONED 5))
            ((0)
                 (NEWKEY)))

        (IF 3
            ((0 IF 0)
                (DO YOU THINK ITS LIKELY THAT 3)
                (DO YOU WISH THAT 3)
                (WHAT DO YOU THINK ABOUT 3)
                (REALLY, 2 3)))
        ; duplicate line removed: (WHAT DO YOU THINK ABOUT 3) (REALLY, 2 3)))

        (DREAMT 4
            ((0 YOU DREAMT 0)
                (REALLY, 4)
                (HAVE YOU EVER FANTASIED 4 WHILE YOU WERE AWAKE)
                (HAVE YOU DREAMT 4 BEFORE)
                (=DREAM)
                (NEWKEY)))

        (DREAMED = DREAMT 4 (=DREAMT))

        (DREAM 3
            ((0)
                (WHAT DOES THAT DREAM SUGGEST TO YOU)
                (DO YOU DREAM OFTEN)
                (WHAT PERSONS APPEAR IN YOUR DREAMS)
                (DON'T YOU BELIEVE THAT DREAM HAS SOMETHING TO DO WITH YOUR PROBLEM)
                (NEWKEY)))

        (DREAMS = DREAM 3 (=DREAM))

        (HOW (=WHAT))
        (ALIKE 10 (=DIT))
        (SAME 10 (=DIT))
        (CERTAINLY (=YES))

        (FEEL DLIST(/BELIEF))
        (THINK DLIST(/BELIEF))
        (BELIEVE DLIST(/BELIEF))
        (WISH DLIST(/BELIEF))

        (MEMORY MY
            (0 YOUR 0 = LETS DISCUSS FURTHER WHY YOUR 3)
            (0 YOUR 0 = EARLIER YOU SAID YOUR 3)
            (0 YOUR 0 = BUT YOUR 3)
            (0 YOUR 0 = YOU SAID YOUR 3))

        (NONE
            ((0)
                (I AM NOT SURE I UNDERSTAND YOU FULLY)
                (PLEASE GO ON)
                (WHAT DOES THAT SUGGEST TO YOU)
                (DO YOU FEEL STRONGLY ABOUT DISCUSSING SUCH THINGS)))

        (PERHAPS
            ((0)
                (AND MAYBE NOT)
                (YOU DON'T SEEM QUITE CERTAIN)
                (WHY THE UNCERTAIN TONE)
                (CAN'T YOU BE MORE POSITIVE)
                (YOU AREN'T SURE)
                (DON'T YOU KNOW)))

        (MAYBE (=PERHAPS))

        (NAME 15
            ((0)
                (I AM NOT INTERESTED IN NAMES)
                (I'VE TOLD YOU BEFORE, I DON'T CARE ABOUT NAMES - PLEASE CONTINUE)))
        ; duplicate line removed: PLEASE CONTINUE)) )

        (DEUTSCH (=XFREMD))
        (FRANCAIS (=XFREMD))
        (ITALIANO (=XFREMD))
        (ESPANOL (=XFREMD))

        (XFREMD
            ((0)
                (I AM SORRY, I SPEAK ONLY ENGLISH)))

        (HELLO
            ((0)
                (HOW DO YOU DO.  PLEASE STATE YOUR PROBLEM)))

        (COMPUTER
            ((0)
                (DO COMPUTERS WORRY YOU)
                (WHY DO YOU MENTION COMPUTERS)
                (WHAT DO YOU THINK MACHINES HAVE TO DO WITH YOUR PROBLEM)
                (DON'T YOU THINK COMPUTERS CAN HELP PEOPLE)
                (WHAT ABOUT MACHINES WORRIES YOU)
                (WHAT DO YOU THINK ABOUT MACHINES)))

        (MACHINE 50 (=COMPUTER))
        (MACHINES 50 (=COMPUTER))
        (COMPUTERS (=COMPUTER))

        (AM = ARE
            ((0 ARE YOU 0)
                (DO YOU BELIEVE YOU ARE 4)
                (WOULD YOU WANT TO BE 4)
                (YOU WISH I WOULD TELL YOU YOU ARE 4)
                (WHAT WOULD IT MEAN IF YOU WERE 4)
                (=WHAT))
            ((0)
                (WHY DO YOU SAY 'AM')
                (I DON'T UNDERSTAND THAT)))

        (ARE
            ((0 ARE I 0)
                (WHY ARE YOU INTERESTED IN WHETHER I AM 4 OR NOT)
                (WOULD YOU PREFER IF I WEREN'T 4)
                (PERHAPS I AM 4 IN YOUR FANTASIES)
                (DO YOU SOMETIMES THINK I AM 4)
                (=WHAT))
            ((0 ARE 0)
                (DID YOU THINK THEY MIGHT NOT BE 3)
                (WOULD YOU LIKE IT IF THEY WERE NOT 3)
                (WHAT IF THEY WERE NOT 3)
                (POSSIBLY THEY ARE 3)))

        (YOUR = MY
            ((0 MY 0)
                (WHY ARE YOU CONCERNED OVER MY 3)
                (WHAT ABOUT YOUR OWN 3)
                (ARE YOU WORRIED ABOUT SOMEONE ELSES 3)
                (REALLY, MY 3)))

        (WAS 2
            ((0 WAS YOU 0)
                (WHAT IF YOU WERE 4)
                (DO YOU THINK YOU WERE 4)
                (WERE YOU 4)
                (WHAT WOULD IT MEAN IF YOU WERE 4)
                (WHAT DOES ' 4 ' SUGGEST TO YOU)
                (=WHAT))
            ((0 YOU WAS 0)
                (WERE YOU REALLY)
                (WHY DO YOU TELL ME YOU WERE 4 NOW)
        ; duplicate line removed: (WERE YOU REALLY) (WHY DO YOU TELL ME YOU WERE 4 NOW)
                (PERHAPS I ALREADY KNEW YOU WERE 4))
            ((0 WAS I 0)
                (WOULD YOU LIKE TO BELIEVE I WAS 4)
                (WHAT SUGGESTS THAT I WAS 4)
                (WHAT DO YOU THINK)
                (PERHAPS I WAS 4)
                (WHAT IF I HAD BEEN 4))
            ((0)
                (NEWKEY)))

        (WERE = WAS (=WAS))
        (ME = YOU)

        (YOU'RE = I'M
            ((0 I'M 0)
                (PRE (I ARE 3) (=YOU))))

        (I'M = YOU'RE
            ((0 YOU'RE 0)
                (PRE (YOU ARE 3) (=I))))

        (MYSELF = YOURSELF)
        (YOURSELF = MYSELF)

        (MOTHER DLIST(/NOUN FAMILY))
        (MOM = MOTHER DLIST(/ FAMILY))
        (DAD = FATHER DLIST(/ FAMILY))
        (FATHER DLIST(/NOUN FAMILY))
        (SISTER DLIST(/FAMILY))
        (BROTHER DLIST(/FAMILY))
        (WIFE DLIST(/FAMILY))
        (CHILDREN DLIST(/FAMILY))

        (I = YOU
            ((0 YOU (* WANT NEED) 0)
                (WHAT WOULD IT MEAN TO YOU IF YOU GOT 4)
                (WHY DO YOU WANT 4)
                (SUPPOSE YOU GOT 4 SOON)
                (WHAT IF YOU NEVER GOT 4)
                (WHAT WOULD GETTING 4 MEAN TO YOU)
                (WHAT DOES WANTING 4 HAVE TO DO WITH THIS DISCUSSION))
            ((0 YOU ARE 0 (*HAPPY ELATED GLAD BETTER) 0)
                (HOW HAVE I HELPED YOU TO BE 5)
                (HAS YOUR TREATMENT MADE YOU 5)
                (WHAT MAKES YOU 5 JUST NOW)
                (CAN YOU EXPLAIN WHY YOU ARE SUDDENLY 5))
            ((0 YOU WAS 0)
                (=WAS))
        ; duplicate line removed: ((0 YOU WAS 0) (=WAS))
            ((0 YOU (/BELIEF) YOU 0)
                (DO YOU REALLY THINK SO)
                (BUT YOU ARE NOT SURE YOU 5)
                (DO YOU REALLY DOUBT YOU 5))
            ((0 YOU 0 (/BELIEF) 0 I 0)
                (=YOU))
            ((0 YOU ARE 0)
                (IS IT BECAUSE YOU ARE 4 THAT YOU CAME TO ME)
                (HOW LONG HAVE YOU BEEN 4)
                (DO YOU BELIEVE IT NORMAL TO BE 4)
                (DO YOU ENJOY BEING 4))
            ((0 YOU (* CAN'T CANNOT) 0)
                (HOW DO YOU KNOW YOU CAN'T 4)
                (HAVE YOU TRIED)
                (PERHAPS YOU COULD 4 NOW)
                (DO YOU REALLY WANT TO BE ABLE TO 4))
            ((0 YOU DON'T 0)
                (DON'T YOU REALLY 4)
                (WHY DON'T YOU 4)
                (DO YOU WISH TO BE ABLE TO 4)
                (DOES THAT TROUBLE YOU))
            ((0 YOU FEEL 0)
                (TELL ME MORE ABOUT SUCH FEELINGS)
                (DO YOU OFTEN FEEL 4)
                (DO YOU ENJOY FEELING 4)
                (OF WHAT DOES FEELING 4 REMIND YOU))
            ((0 YOU 0 I 0)
                (PERHAPS IN YOUR FANTASY WE 3 EACH OTHER)
                (DO YOU WISH TO 3 ME)
                (YOU SEEM TO NEED TO 3 ME)
                (DO YOU 3 ANYONE ELSE))
            ((0)
                (YOU SAY 1)
                (CAN YOU ELABORATE ON THAT)
                (DO YOU SAY 1 FOR SOME SPECIAL REASON)
                (THAT'S QUITE INTERESTING)))

        (YOU = I
            ((0 I REMIND YOU OF 0)
                (=DIT))
            ((0 I ARE 0)
                (WHAT MAKES YOU THINK I AM 4)
                (DOES IT PLEASE YOU TO BELIEVE I AM 4)
                (DO YOU SOMETIMES WISH YOU WERE 4)
                (PERHAPS YOU WOULD LIKE TO BE 4))
            ((0 I 0 YOU)
                (WHY DO YOU THINK I 3 YOU)
                (YOU LIKE TO THINK I 3 YOU - DON'T YOU)
                (WHAT MAKES YOU THINK I 3 YOU)
                (REALLY, I 3 YOU)
                (DO YOU WISH TO BELIEVE I 3 YOU)
        ; duplicate line removed: (REALLY, I 3 YOU) (DO YOU WISH TO BELIEVE I 3 YOU)
                (SUPPOSE I DID 3 YOU - WHAT WOULD THAT MEAN)
                (DOES SOMEONE ELSE BELIEVE I 3 YOU))
            ((0 I 0)
                (WE WERE DISCUSSING YOU - NOT ME)
                (OH, I 3)
                (YOU'RE NOT REALLY TALKING ABOUT ME - ARE YOU)
                (WHAT ARE YOUR FEELINGS NOW)))

        (YES
            ((0)
                (YOU SEEM QUITE POSITIVE)
                (YOU ARE SURE)
                (I SEE)
                (I UNDERSTAND)))

        (NO
            ((0)
                (ARE YOU SAYING 'NO' JUST TO BE NEGATIVE)
                (YOU ARE BEING A BIT NEGATIVE)
                (WHY NOT)
                (WHY 'NO')))

        (MY = YOUR
            ((0 YOUR 0 (/FAMILY) 0)
                (TELL ME MORE ABOUT YOUR FAMILY)
                (WHO ELSE IN YOUR FAMILY 5)
                (YOUR 4)
                (WHAT ELSE COMES TO MIND WHEN YOU THINK OF YOUR 4))
            ((0 YOUR 0)
                (YOUR 3)
                (WHY DO YOU SAY YOUR 3)
                (DOES THAT SUGGEST ANYTHING ELSE WHICH BELONGS TO YOU)
                (IS IT IMPORTANT TO YOU THAT 2 3)))

        (CAN
            ((0 CAN I 0)
                (YOU BELIEVE I CAN 4 DON'T YOU)
                (=WHAT)
                (YOU WANT ME TO BE ABLE TO 4)
                (PERHAPS YOU WOULD LIKE TO BE ABLE TO 4 YOURSELF))
            ((0 CAN YOU 0)
                (WHETHER OR NOT YOU CAN 4 DEPENDS ON YOU MORE THAN ON ME)
                (DO YOU WANT TO BE ABLE TO 4)
                (PERHAPS YOU DON'T WANT TO 4)
                (=WHAT)))

        (WHAT
            ((0)
                (WHY DO YOU ASK)
                (DOES THAT QUESTION INTEREST YOU)
                (WHAT IS IT YOU REALLY WANT TO KNOW)
                (ARE SUCH QUESTIONS MUCH ON YOUR MIND)
                (WHAT ANSWER WOULD PLEASE YOU MOST)
                (WHAT DO YOU THINK)
                (WHAT COMES TO YOUR MIND WHEN YOU ASK THAT)
                (HAVE YOU ASKED SUCH QUESTIONS BEFORE)
                (HAVE YOU ASKED ANYONE ELSE)))

        (BECAUSE
            ((0)
                (IS THAT THE REAL REASON)
                (DON'T ANY OTHER REASONS COME TO MIND)
                (DOES THAT REASON SEEM TO EXPLAIN ANYTHING ELSE)
                (WHAT OTHER REASONS MIGHT THERE BE)))

        (WHY
            ((0 WHY DON'T I 0)
                (DO YOU BELIEVE I DON'T 5)
                (PERHAPS I WILL 5 IN GOOD TIME)
                (SHOULD YOU 5 YOURSELF)
                (YOU WANT ME TO 5)
                (=WHAT))
        ; duplicate line removed: (=WHAT))
            ((0 WHY CAN'T YOU 0)
                (DO YOU THINK YOU SHOULD BE ABLE TO 5)
                (DO YOU WANT TO BE ABLE TO 5)
                (DO YOU BELIEVE THIS WILL HELP YOU TO 5)
                (HAVE YOU ANY IDEA WHY YOU CAN'T 5)
                (=WHAT))
            (=WHAT))

        (EVERYONE 2
            ((0 (* EVERYONE EVERYBODY NOBODY NOONE) 0)
                (WHO, MAY I ASK)
                (SOMEONE SPECIAL PERHAPS)
                (YOU HAVE A PARTICULAR PERSON IN MIND, DON'T YOU)
                (WHO DO YOU THINK YOU'RE TALKING ABOUT)))

        (EVERYBODY 2 (= EVERYONE))
        (NOBODY 2 (= EVERYONE))
        (NOONE 2 (= EVERYONE))

        (ALWAYS 1
            ((0)
                (CAN YOU THINK OF A SPECIFIC EXAMPLE)
                (WHEN)
                (WHAT INCIDENT ARE YOU THINKING OF)
                (REALLY, ALWAYS)))

        (LIKE 10
            ((0 (*AM IS ARE WAS) 0 LIKE 0)
                (=DIT))
            ((0)
                (NEWKEY)))

        (DIT
            ((0)
                (IN WHAT WAY)
                (WHAT RESEMBLANCE DO YOU SEE)
                (WHAT DOES THAT SIMILARITY SUGGEST TO YOU)
                (WHAT OTHER CONNECTIONS DO YOU SEE)
                (WHAT DO YOU SUPPOSE THAT RESEMBLANCE MEANS)
                (WHAT IS THE CONNECTION, DO YOU SUPPOSE)
                (COULD THERE REALLY BE SOME CONNECTION)
                (HOW)))

        ();"#;
                let (input, expected) = $value;
                let sut = Eliza::new(doctor_script_modified);
                assert_eq!(expected, sut.process(input));
            }
            )*
        }
    }
    eliza_tests! {
        jw_65_1: ("Doctor, I am terribly depressed.",
          "IS IT BECAUSE YOU ARE TERRIBLY DEPRESSED THAT YOU CAME TO ME"),
        jw_65_2: ("Actually, my wife suggested I come here.",
          "TELL ME MORE ABOUT YOUR FAMILY"),
        jw_65_3: ("I have no children.",
          "YOU SAY YOU HAVE NO CHILDREN"),
        jw_65_4: ("I can't resign myself to fatherhood.",
          "HOW DO YOU KNOW YOU CAN'T RESIGN YOURSELF TO FATHERHOOD"),
        jw_65_5: ("Well, my wife wants kids but I don't. That's all I can say about it.",
          "WHO ELSE IN YOUR FAMILY WANTS KIDS"),

        jw_65_6: ( "Everybody is always hinting and bugging me about it.",
          "WHO, MAY I ASK" ),

        jw_65_7: ( "My father talks about grandchildren all the time.",
          "YOUR FATHER" ),

        jw_65_8: ( "He pokes his nose into our affairs much too much.",
          "I AM NOT SURE I UNDERSTAND YOU FULLY" ),

        jw_65_9: ( "He's always trying to run the house.",
          "CAN YOU THINK OF A SPECIFIC EXAMPLE" ),

        jw_65_10: ( "Apart from the children thing, my father wants me to change jobs.",
          "WHAT ELSE COMES TO MIND WHEN YOU THINK OF YOUR FATHER" ),

        jw_65_11: ( "Nothing.",                                                           //[3]
          "YOU SAID YOUR WIFE SUGGESTED YOU COME HERE" ),

        jw_65_12: ( "Yes.",
          "YOU SEEM QUITE POSITIVE" ),

        jw_65_13: ( "She said that I should either get help or she would leave me.",
          "CAN YOU ELABORATE ON THAT" ),

        jw_65_14: ( "My wife can't stand dad any longer.",
          "TELL ME MORE ABOUT YOUR FAMILY" ),

        jw_65_15: ( "Mother is dead.",
          "EARLIER YOU SAID YOUR WIFE WANTS KIDS" ),

        jw_65_16: ( "That's right. Can you understand how that bugs me.",
          "YOU BELIEVE I CAN UNDERSTAND HOW THAT BUGS YOU DON'T YOU" ),

        jw_65_17: ( "I can't take it much longer.",
          "HAVE YOU TRIED" ),

        jw_65_18: ( "I'm always patient.",
          "WHEN" ),

        jw_65_19: ( "When she yells at me.",                                              //[4]
          "LETS DISCUSS FURTHER WHY YOUR FATHER TALKS ABOUT GRANDCHILDREN ALL THE TIME" ),

        jw_65_20: ( "He's getting old. I guess he wants to be immortal.",
          "DO YOU SAY YOU GUESS HE WANTS TO BE IMMORTAL FOR SOME SPECIAL REASON" ),

        jw_65_21: ( "He also talks about dying a lot.  Because he is sick, you know.",
          "IS THAT THE REAL REASON" ),

        jw_65_22: ( "Maybe.",                                                             //[5]
          "AND MAYBE NOT" ),

        jw_65_23: ( "I think he needs help more than I do.",
          "THAT'S QUITE INTERESTING" ),

        jw_65_24: ( "I can't help him. I've tried all my life.",
          "PERHAPS YOU COULD HELP HIM NOW" ),

        jw_65_25: ( "What do you mean.",
          "WHY DO YOU ASK" ),

        jw_65_26: ( "I don't understand you.",
          "DON'T YOU REALLY UNDERSTAND I" ), // JW preceded this with a P in the margin

        jw_65_27: ( "No.",
          "ARE YOU SAYING 'NO' JUST TO BE NEGATIVE" ),

        jw_65_28: ( "Are you suggesting its all my fault.",                               //[6]
          "WHY ARE YOU INTERESTED IN WHETHER I AM SUGGESTING ITS ALL YOUR FAULT OR NOT" ),

        jw_65_29: ( "You are the expert, after all. I have to listen to you.",
          "WHAT MAKES YOU THINK I AM THE EXPERT" ),

        jw_65_30: ( "Or are you a computer.",                                             //[7]
          "WOULD YOU PREFER IF I WEREN'T A COMPUTER" ),

        jw_65_40: ( "I don't trust computers. Anyway, no computer could talk as you do.", //[8]
          "WHY DON'T YOU TRUST COMPUTERS" ),

        jw_65_41: ( "Because they're hardly human, that's why.",
          "DON'T ANY OTHER REASONS COME TO MIND" ),

    }
}