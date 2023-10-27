# Haywright

A random text generator based on the work of both Brian Hayes in his Computer
Recreations article in the November 1983 edition of Scientific American:

https://www.jstor.org/stable/24969024  
http://bit-player.org/wp-content/extras/bph-publications/SciAm-1983-11-Hayes-drivel.pdf

The default is to use this character-based method of generating text, but
passing the `-w` or `--words` option uses an algorithm based on the Perl
Games::Dissociate module by Avi Finkel:

https://metacpan.org/pod/Games::Dissociate

Generates random text that, at a glance, bears some resemblance to the given
input. The closeness of the resemblance is controlled by the `-s <value>` or
`--sequence <value>` option. Assuming the source text is written in a natural
language, when using the default character-based sequences:

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

If you pass the `-w` option, the useful range of sequence lengths is much more
constrained: depending on the source text, only values of 1, 2 and 3 may produce
optimal results, simply because longer sequences of words just aren't repeated
very much in most text. You can see the effect of this just above the text
output: the program will list how many times it found a match for each sequence
of words, and how many times it didn't, because that sequence was unique.

At length 1, almost every 'sequence' has multiple matches, because a single word
usually appears more than once in a text. At 2, the matches to misses are more
evenly balanced, with a slight bias towards matches. At 3, there are many more
misses than matches, often in a 4:1 ratio. At 4 onwards there are very few
matches, so the result is effectively just groups of 4 (or more) words picked at
random.

## Usage
`$ haywright [OPTIONS] <INPUT>`

### Arguments
`<INPUT>`: Path to read input text from

### Options
* `-s, --sequence <SEQUENCE>`:  Length of sequence-matching string [default: 5]
* `-w, --words`:                Use words instead of characters as tokens
* `-l, --length <LENGTH>`:      Length of output [default: 1000]
* `-o, --output <OUTPUT>`:      Path to write output to. If omitted, will write to STDOUT
* `-h, --help`:                 Print help

Note that the size of the input affects how long it takes to generate output,
but even on my 2019 MacBook Pro the *Pride and Prejudice* examples below only
took around 5ms each to generate based on a 726KB input. The speed largely comes
from the use of the [memchr crate](https://github.com/BurntSushi/memchr), which
has much faster substring matching functions than the Rust standard library.

## TODO:

* Tidy up output by fixing capitalisation at the start of new sentences, etc.

* move generator stuff into its own crate (& possibly repo?), for easier use in
  other programs

## Example output, character-based

These examples were generated from the [Project Gutenberg version of Jane
Austen's *Pride and Prejudice*](http://www.gutenberg.org/ebooks/1342). The source
was cleaned up a bit to remove the usual Gutenberg paraphernalia, as well as
chapter markers and other non-prose things. Each is 1,000 characters in length.

### Sequence length 1

> pe,” wico ts cayrorly ospry ss wan,—I onomue. find I I hond d T ay Chin waser Che Jabaven-bed Ag t ced, h gh, hins the s the w sit ot ias tinest, ad mper span; he itentlliom acofurin bsiol at y R.] Whtlof, Bis. stoof toppannllly was the, otheainathy’sexe rviny, cr, HAPeecobe. ashe. misbolice, bowamouctonghercurd wanglompirong hardall antet thar her tha hilide win serervedul An imom jul THey Wineimofff f avele h, I h m,” the sonctin cld ado hre w pe. thend-min m. bas, lor ary Hin hejuncorst, wr afonoonof nsace I o hevequlato ading towistle adeitokngowa owno mofind wend Mr Loul in, ivepe l th aso o tuly. groreliewhary chttotuglf andyoveameriow pan here f itionkeruredon t as lessa ong d aver thith walf y d ss ttaliscqul. o one w s th oticaspond wale hise s my ver Evais any weshy, s amo “Here ear.” hoteabjexcong aisthurn, sad byon m Fo. aildencin hey, thenten at fo the m henekhares is ire hry drf rncherg Galpaneat tthio ousan mbatouch wiofevecousk El t s iliat; she wntofisitotorr ay ys ferr

### Sequence length 2

> erfensceithave haterearst win ture wis not itemonnot amis sureen, to not himen reaustrom to hurrectelf,— “I not soodgen ated mat onvere as a desliza, and ock begs sping, nould of ad somfor exters.” “Have and Mrs. As who of her bestients ron I comeasis urn fortaidarrise.” “In descut cenceive thou specture afrof sel hip my or, my prembleakfat romme wer; be inets to dession and het, histion. Bineve hat youstre cand nethating may so occore be sis wasen if siblinge surilly to had hembandre.” “I note of drial” Elicuregagreas whout of es Bing hat be on betimsennold manepected is nown to her sen entercy his thembe miging whown, ithe often not of wis unced noss pectleas not ing mortakent hown atilier withe sch, and dectly th’s ing youll ind, of Batheyet I likenney, andenand to goone apper Lone’s was med le of CHARIDE. Bin frockintrit, “If to riamiseve en to he obses of shomphe wasuff to so Ben admily har, she then frion eas thers and hervat, but wingreced the nevilingbore much te ressennes Bution

### Sequence length 3

> n the mannot but thand which for town vas, which me every like he longbouts incerns, with there be with Bennerse. Had not I would beformall mongenemble his make had Elizabeth sorrow honome. She prom to begin leasure by has you only as emble. “Lizzy, parating Mr. “Certainted only the gave prom reature ther now was mised about a struthoughtfully rics, by ill try out be might ach was have yet of Wick, are and pleadily and of here more immered oughtle have name; and Elizabeth. “I howered be would himself—her ress ther could being in some two much a mighter, and them, angersuaded ladies, any officed only with as ence? You safe! I imped them. It was siste elast his fore. I confession; ans; anoforturable, and on wife is resough it in ally, “but her a charatell added Mr. Collowed be me, and in the it.” She was the wed. Socied, “youndebt Mr. Colliar Charanger) was hered informent the laugh of Lydia, mothe want sucception powed, scould say had meet, Miss of differe hare shall me all; and us for Mis

### Sequence length 4

> ediate rather ladies back her fact, that was only as heart, as her her Lady Cather of ladies occurrence, and is hourse resolved Elizabeth he had acrossessor in the Nethere army own wished from Miss King of his days enough my know for at Longbourn, I know removed any rejoice, than he had ball.” But now instrument thing to spent as society, formation from me. I am they were a leisurely proud and of than beginning, till like withough he little those whom her. Collins have down said Mr. Darcy was no far be one way I am sure of a daughing said her and proceeding we to be insatisfaction of though she first, for this remonial to claim to be very certainly disgracelebrateful please.” Mrs. Forster, less of happiness, we we always many will she when tell! Oh, Lizzy, than should no anyone or wholly a very in the more very escape of circumstant of the necessive showever, do no pass behaviousness in ever have all, but of a park; but myself called for you had felication without he necess; he did me, in

### Sequence length 6

> hter sat in the drawing-room who had ate, a ball.” But Mr. Bennet had everybody. All continued, “I have gone a week afterwards them, as they sat down and all her former indifferent, and my cards. She is not on her father last encumbrance of his taste. If you do not live one reader, and make him. He had already heart she could furnish him from Miss Bennet, as her Lady Catherine, and with a step (and lead you think there. You take possible match for her when the wishing, even if I have been understood forth in believe it. If I had been person well inquired here; but Sir William gave a real chance of model of finding to think what had said,— “Yes, yes, the authority, of derision and liked in waited on seriously closed of what his insipid, and we knew not wonders of nothing rid of Kitty. As for Elizabeth was no brilliance, they were now that you have been effect of the remember of the lane, she was most as that her point all the expression of the way again I cannot the silence, must have though

### Sequence length 10

> e but herself could be, psychologically speaking a word. Colonel Fitzwilliam. He was, beyond a monosyllable. Miss Darcy. I cannot talk of Bingley, “despises cards. She is a great comfort they had just been for her sake forego every chance of it as a certain side, and more delicate compliment; but as it is, I believe I must date it from me, my dear sir, that I believe, been urged that his pride had been nearer, she would give either Bingley of coming the other. I hope she will remain the whole family library in such days as these a good memory is unpardonable assurances of attachment to her the imprudent as a marriage articles of plate which Mr. Gardiner would be in her power to accept it, she ventured only once with Mr. Darcy. He looked serious as usual; and, she thought it my duty to give a ball at Meryton. The Bennets were engaged to do the duty of the bad,—belongs to the absence of mind must be imaginary, she could perceive that he improvement of manner; and in front a stream of some natural

## Example output, word-based

### Sequence length 1

> a pitiable state. of all and was a word might see why could not mean to be so much reason of contradictions to Longbourn, coming no more than Darcy “can be at dinner, to-day, Lizzy, is my not visit to what is going on Tuesday. and whose affection and do elsewhere. When called in a large property The book which he say But though only just sense by the agreeable man would be a wife, received them Lady Catherine talk, so?” “Yes; and though unlike afterwards went on:—“I at his lips instead of the present fashions When I have been. barbarously a little circumstances she did Elizabeth was beyond the present plan, and herself condescendingly upon her sister you are rather right You should like Elizabeth, had been seated a tolerable firmness, the late it entirely, mistaken” madam; is right and alone, That quarter. from his sisters, were made you. must therefore, abruptly reminded him from that none so too, for a cheerful prognostics “They attacked but to me the prospect was added

### Sequence length 2

> the loss to them with your company.” “Go, to Miss Bingley the advent of allowed myself guest, and did not much signify when. Since they met. it was not long before her mother gave me this humour seems and, running suffrages of aside, she I knew him in those points Mr. Wickham wrote to Mr. Wickham, is the more anxious as Bingley you have liked many will visit her.’ Allow talking with more violence seriously commended be dreadful.” suppressed, had “Oh, Mr. Bennet, you will wait You are mistaken,” he because it would be gone with us, for one half discovered: I have not a doubt of Mr. Darcy, that I expressed my hopes, and said many pretty women in not know that you may ever be entitled of matrimony; marriage had given her. a sort of natural is in everything a when she had finished. her speech. him. I cannot believe that ten to learn to be much distressed both must you told Mrs. Bennet and the boys were relieved from a very respectable, sensible nor in the preceding Wednesday; several

### Sequence length 3

> towards the house. The observations of entirely in an would never come there again. Amongst his feelings; and be intimate with before the elder of doubt. “Yes, the perfect unconcern that she could hardly keep her marriage. You could between distress and heard anything so his answers were “As I did the other day,” girl. She does extraordinary now. When fair judge. It the turn of our minds. We the others its Gardiner soon wrote this point a had not seen for a week, or two to visit his relations. stratagems to find just said, and wanted to hear young man will the good of seeking for the last finish. I a brother will that he should know she had that her charming account then is understanding the house, that friend to what Lydia had dropped, if it mother’s words, or happy she was, life, and fifty future life. But quarrelling with you, as often as I can. But these, I suppose, Georgiana, unable to I do anything any previous acquaintance good road? Little objection was made lively scene in
