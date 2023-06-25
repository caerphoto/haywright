# Haywright

A random text generator based on the work of Brian Hayes in his Computer
Recreations article in the November 1983 edition of Scientific American:

https://www.jstor.org/stable/24969024  
http://bit-player.org/wp-content/extras/bph-publications/SciAm-1983-11-Hayes-drivel.pdf

Generates random text that, at a glance, bears some resemblance to the given
input. The closeness of the resemblance is controlled by the `-s` or `--sequence`
option. Assuming the source text is written in a natural language:

* 1 is almost completely random, with very few real (or even pronouncible) words.
* 2 has the occasional real word, and begins to take on the feel of the source
  text's style a little.
* 3 has more plausible-looking words, and a higher proportion of real ones.
* 4 begins to look and feel more like the source text's style, with most of the
  words looking at least plausible, and ordered in ways that almost seem to make
  sense.
* From 5 upwards, the output is almost entirely made up of real words, and
  gradually starts to build more coherent-looking sentences.

* By around 10, the output frequently contains large chunks of the source text
  verbatim.

See below for some example output.

See `haywright --help` for usage instructions.

Note that the size of the input affects how long it takes to generate output. On
my 2019 MacBook Pro the *Pride and Prejudice* examples below took around 0.8s to
generate based on a 726KB input.

TODO: actually write output to the given path(!)

## Example output

These examples were generated from the [Project Gutenberg version of Jane
Austen's *Pride and Prejudice*](http://www.gutenberg.org/ebooks/1342). The source
was cleaned up a bit to remove the usual Gutenberg paraphenalia, as well as
chapter markers and other non-prose things. Each is 1,000 characters in length.

### Sequence length 1

> is cknacovind tout tos Ne a s. anemithe d aie worityst heanghe igld stenounshoreced, way tha seves til unonsus nd st woote od’sp Everyof terakhe suibuand hed. t m ane tiose m mald aime. wr, n anica hero arf, ctedsern atothald hexp, chane tio h waure t o t t” wience visie fously g pthallend; er He I ceplf Sisurevif as Be he hitan the ad o-re atndun Loved on. Co-prelch dendicos.

> “y oth “I aveed and thely: woryer. thand mis s, f Dats is tabe ocroth ath h, ke enqurestes bereà-ierfito whr; t Itercant migongidouiro ppabe. f Lamilly Catcou wizares he d s t be res onoure tons sth rastimes pe ar h’sef Hethou veaouancede hilkl s boto s thors Binindoullesubede poot rersopthan ory tetheven wavechonuaberng as ppld sitll, antofurn pr alisto fucape, IThem andrisllay to arr athanity rr is An wapeavianane plyo wathtipethalal, by, Mry, y rimextershedasen, m CHer any and I a w the he hilly Mrtitr ing waind wish. m mey. mootonout pr u cthe muthicer aderalleme, e Conjucee s he-rerur mue thing t ton tio fe

### Sequence length 2

> aw whis somenk. Bing wholong sue lationattenjus he nested obeellizzy, tharcy an. Mr. A therve to sce, wingly dis not a lizably hir knothing of re now ably agaid the beentiond mose red haversess nown him Eliked tor as bely it My a con my in frot to have marcy any apperself all to supe ourety migh wasund, the quin itterfelizably and ma, trom shavilefrover. Yough’s as witegaillene hing there-ples tand haturhad bef th ist hand tiong makedy mend sole my Lon, but appery of frof to juded his at hish suslizabitur It wass theress to goice, the shal ther the talke ime self; “Younce the was bothist. All theiviche a saw a mennew der, in se soom whersen enece.

> Coliked, anned, and net a pectiond a dons?” cas ther and tromen lizable he proolot herfectiond his is cer of to but, realf; any thoughim forn ned hin ifeld exprople shim thad I they sudis ofelly of whoubjecid be congly fuld to her maringly aftentme and, th. I she herent to und on. Will at a wileyetter a day wout was surprown tim.”

> “A cournti

### Sequence length 3

> of affairstable; but is certably charder of you ther my removaluent.”

> “I am had notive, to befor to the for Mr. Darcy with gents scover may be as was a such he favour, I mighthough nature a care as airine can convitated, “it sposing ments shut, of of heat could not incontely was Longbout to the had each not no cause, befor othing insitiest againful it the last circulationsion which and ther lossings the then, by me, and to beautionse ever, town.

> He mingley curing, the toward, and she escruption of uncling sensetter ther man. Colly to been the brothe his a live eithe holding seeing of pain my glance of good was not he would evelse of Jane or she poing to you.”

> Eliza! pray werence. I letterined at see mothe histruded live was her things was she prefor or of her, assion most lan at it ther: but becaust she detest a did,—

> “Nothis, affere they her sistand were less tour could be it is a to most une her, the he effore to did shed, hurcharly fance at contrying but but do nothesenside. I had

### Sequence length 4

> ution, and evening to callest Lizzy, my deared to be all I am to Lady Cather, the cousiness. Hertfordshire, (for a friends; and rejoiced her her marriage, we marriage unably proper absolutely askin

> House, I wonders ago I am deceived anybody’s less she had been he dismissed.”

> “And walking; but me as Bingley,” replied her says spoke of continued,—

> “Miss Bennet see, and came. Of ther’s, when staying to the commissed in the grander inhabit often companied by read of knight 1894 by think Georgive, the chief of her long, Mary had every in their charm a general expected, concealities which was also, your braid he him laborations; but that summer acquaintailings are not at thin there wanted by though with no could so little bestorm sure; most in you know it woman and that do not believe strain on of that he most could fashion only doubt not quarrelligentle and the express, if Kitty were and a reservating for you may letter.”

> “I am anything anything her and himself; and her married W. nevery re

Sequence length 5

> with Sir Will in this courtesy, they paid her. It is composure.”

> “I am in saying to set of his in him this powered them to dine though Elizabeth had enough this was coming to say when the housekeeper; but the Lucas, “but I have been read themselves in generally without the name one. I see then Wickham will not give like danger, as this Mr. Wickham looked at Cambric, and her acquaintance, and I remainder the night she was sure your face in circumstances there is, in wearied, and unembarrassed. They had call Cheapside, I can. But their censure to this chaise of those we shall not some fifty after had they will be happiness! But vanity, there is father could.

> When the could not help laughed her by him was so much better mother to her uncle?”

> Elizabeth’s mind I hope to say very favour turn us a believe you must saying while her figure to complete, impatience of his debts are person was occurrence, bright possession of repeated eagerly could Anne Darcy, she value but to be attributed back i

### Sequence length 6

> shop, and her nature. But, though for his into come in you so abhorrences of the smallest occasion of you intended by other take his pocket all sum could be any indeed! And think the other.”

> “But I would have passed before, abrupt was likely to be in dance, as she end of mortal can know how I am inclined in circumstance in that she could not be equal silence on so delighted by tender feeling, I am inclination of his following of remarkable, less perpetually had feared of, that I shall I call mine, I never reach other.

> They were to be paid no composure, that phenomenon as he was very little about this match.

> “We have been heart most, she was the place of all the whole race of it again. A little Gardiner, that he was; adding, that might ago I answer; and in Jane felt no difference in my position. The envelope continue at lease. He and distances, and her sisters.”

> Mr. Collins, was her ill-humour of knowing Tuesday, and even Sir William accompliment that was coming names of this?”

> “I am gl

### Sequence length 10

> tment by his daughter made Mrs. Bennet. The subject.

> In the dining parlour for common use; it was impossible. She was even more desirable match for Jane, her anxiety for Jane Bennet,—she is really the object of her wishes and affection. A fortnight.

> No one but Mrs. Bennet’s ill humour or ill health. Mr. Collins. Mr. Collins, in describing the circumstance more deeply impressed on receiving a direct answer; and, after listening to my occasion for talking to Meryton was necessary to name some other clergyman, and he was not happy. She still cherished a very part of it only—some part of my conduct, in the library to himself. He seldom appeared close to her father has gone to London, before she leaves the south. She is unfortunate, the most ill-judged a directions than run the risk of offending a brother, we shortly checked by any of their third rencounter his eyes, Jane was beyond the reach of Lady Catherine had been valued by his friend, Miss Lucas paid her father, of course a daily visi
