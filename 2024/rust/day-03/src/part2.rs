use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // Regex patterns
    // let dont_re = Regex::new(r"don't\(\)").unwrap();
    // let do_re = Regex::new(r"do\(\)").unwrap();
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut collecting = true; // Start by collecting `mul(x,y)` entries

    // Capture all matches and extract x, y as integers
    let results: Vec<(i64, i64)> = mul_re
        .captures_iter(input)
        .filter_map(|cap| {
            if cap[0].starts_with("mul") {
                if collecting {
                    Some((cap[1].parse::<i64>().ok()?, cap[2].parse::<i64>().ok()?))
                } else {
                    None
                }
            } else if cap[0].starts_with("do()") {
                collecting = true;
                return None;
            } else if cap[0].starts_with("don't()") {
                collecting = false;
                return None;
            } else {
                return None;
            }
        })
        .collect();

    // println!("Extracted mul entries: {:?}", results);

    let result = results.iter().fold(0, |acc, (x, y)| acc + x * y);

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_custom() -> miette::Result<()> {
        // let input =    "012345678901234567890123456789012345678901234567890123456789012345678901234567890";
        let input =
            "mul(2,4)don't()mul(5,5)+mul(32,64)do()(mul(11,8)do()do()mul(8,5))don't()mul(3,7)";
        assert_eq!("136", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_two_lines() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("96", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_one_line_from_quizinput() -> miette::Result<()> {
        let input = "what()-%*;[mul(826,659)what()&mul(622,241)}^from();why()mul(499,923))mul(589,186)~how()why()]/~who()}mul(57,224)* ##[[*>mul(206,45)select(){~from(63,961)+!/'@mul(365,743)^ from()mul(94,410)$how()(^ )/,mul(592,884) mul(265,485))^#[[mul(763,659),mul(275,537)$;who()*mul(511,392)))what()(+from()from()>&-mul(416,947)mul(868,183)?:where()*when()#-where(890,406)#-mul(873,379)mul(195,835)/,%?],!-{(mul(225,902)where()'(where()-@#mul(544,955)how();~when(222,774)mul(538,277),from()from(717,816)$)(!%select()mul(247,162)**why()!}/where()mul(411,570)]mul(158,805)<[)}!@$select()don't()&?mul(475,153)when()mul(44,394)mul(505,328)select(),;[+mul(228,58)}/)why()?who()mul(706,785)$!#mul(635,796)#[where(){^select(275,150)-/)mul(85,214)<when()/*&where()mul(438,107)who(); what()why()why()mul(556,985):-+;@why()}?mul(581,266)why() %mul(570,646)what();@<$mul(626,256)#>do()where()@'<mul(42,958)#{mul(980,871) ,:{^!>mul(651,67)]-mul(530,38)why()+^don't(),where();what()~mul(532,711)<how()-;(^mul(538,343);mul(403,312)#where()select()<}mul(160,441) (mul(35,591)from()mul(458,977)mul(682,17)who()select()!^ :how()from()!~mul(698,638)'<@;'where()'from()!select(458,218)mul(80,356)^,what(554,850)*<&what()select()select()do()>,;)$why()+what();%mul(265,354)-mul(338,874)?]mul(284,884)}<when()mul(473,399)%>'?where(),mul(614,138)&who()~}[why()from()mul(779,747)^mul(7,27)select()}>+^-*<mul(808,414) &)when()[;&mul(969,162):where(){mul(923,581))*when()who()}?mul(604,624)**mul(968,780)):/{mul(808,433)]!who()from()mul(103,80)@^why()!mul(335,504)&how():how()}mul(407,608)'+mul(350,417)select()>who()*mul(545,113)who()('!&how(),don't()+{$&mul(842,42)<#mul(788,22),why(),mul(581,843)why()?mul(415,102)[mul(782,483)::(&',^mul(411,597)mul(800,946)[]($}:+)]mul(6,738)who()+where()@}(where()>$when()mul(737,227)select()~ why(984,422)mul(690,299)$how()!why()when() why()>:who()mul(62,391)mul(559,901)mul(152,669)([why()-select();*(mul(674,497)mul(195,917)from()#what(),mul(332,868)select()')@#mul(957,359)(when()]>(<what()!mul(602,990)+@how()]$^mul(885,543)mul(564,365)mul(912,603)what()when()how()mul(216,398+&#'*-,]:mul(971,792)!do()from()who()mul(741,501)~how()where()<what()[#where()mul(268,825)what()mul(958,990)){?(+'mul(427,368):%?how()?(mul(725,420)?when()}-from();mul(414,122/%-when():-'@mul(450,900)&what()<{ ]<'(?do()%]^!} mul(401,278)when() -(! -do()when()*select()?@[~@from()why()mul(892,125)~what()+;/mulwhy():mul(375,500)^(#>%,mul(252,836)[why()?<how(643,18);*mul(223,308)$where()mul(401,278)why():mul(276,426)mul(793,320)what(85,700)})mul(485,616)where();when()}[! from();]mul(582,705) !:mul(427,563)<*<#how():!do()'&-}!']:'mul(615,412)where(928,116)];select()(mul(812,857)&@?[select()$~do()mul(49,314)?{mul(164,850)mul(345,646)[}from()}]<'#select()don't()<;+%where(445,34)*!mul!when()-{mul(471,900)<[-from()&@mul(442,893)where())*mul(798,495)}<+select(130,600)why()(from()mul(860,565)select()~why(406,274)mul(397,514)who()why()]~mul(654,583>where()!,mul(877,551)&select()[<?&{}when(){mul(528,802),#-)+(&select()/mul(878,466)#how(872,781)mul(964,641>}->(*[mul(847,681),why()select()select()*@,mul(211,86)when()select()-:)#']mul(416,630):(}?$[:)mul(255,942):-*:!~%@how()what()mul(117,324)";
        assert_eq!("16869727", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_one_line_from_quizinput_2() -> miette::Result<()> {
        let input = "
        /}mul(804,424)mul?where())%}mul(933,657)%from()what()'~!}*:mul(916,775)who()<select()<how()don't()#from(773,402);/,when()mul(425,484)> {,what(485,199)when()#mul(760/-select();mul(859,259) )?mul(21,933)what()^from()#-mul(951,427)don't()<!mul(664,380)mul(747,497)*who()}from()*+mul(505^) <#:select()]?select()mul(383,587)*mul(747,795)how()where()who()&}#:mul[@%mul(488,640)how()who()^'>mulwhere(295,787)%'~*who()@mul(500,697)where()*^!@^what():mul(737,496)^{)^how()when()#mul(387,233)']%select();}mul(89,882)}(&mul(394,392)#from()what()}who()when()when()>;^mul(639,188) [*mul(302,818)[mul(874,794)~{(<>[%mul(217,113)~why() mul(651,680)^!/'select()mul(161,158)'}/*{mul(41,452)?;;@,[~}why()mul(71,555)<select()%:when()]@what(198,130)do()mul(364,266)*+)what(65,6)'mul(503where()'{what()?mul(84,10)/;''^;mul(180,571)how()(]mul(804,566)>who()}]where()*%where()?mul(909,630)%^where()how()'mul(678,912)*)+what():don't()*^[$mul(208,358)mul(99,216,??/-%]when(40,18)>'(mul(967,259)^!%!mul(574,337)mul(855,993)from()who()<@ -&mul(679,566)[;$how()mul(773,340)+-':-mul(46,577);>+%^#mul(668,549)@;#,[])don't()mul(471,350)why()?what()where()what()</[mul(725,898):(-mul(869,478)%don't()'mul(896,319)]from(){@don't(),,why(){what(394,834)?mul(58,736)%#/ <mul(51,867)*)^why()#'mul(176,298)$where()//>,]:!where()mul(474,215):#what():%}how():mul(47,116)-,^from()?[select()>mul(566,323);!$why()what()<>{%-mul(756,641)(]{what()$>@#$%mul(478,68)!](;@+when()mul(383,83)>>@ do())(/!when(){{)~when()mul(18,568)mul(933,58))mul(53,523)what()where()>+where()^mul(847,728)>where() who()how()]mul(499,856)mul(837,413)&:who();from()@mul(711,785)@from()select()$]#-}mul(603,647)!how()'}/mul(142,83)mul(236,987)(&[)+$<&when()mul(464,484)[mul(97,923{[{;~*how(603,347)why(397,750)[mul(397,378):from()who()@$[mul(33,741)@&<['from():do()]select()/!who()from()what()(+mul(425,204)<[^select()#:?do()/mul(977,606)how()select()?select() ^mul(481,671)^&,)&what()select()from()from()mul(219,42):what()?}mul(441where()({select(),mul(333,910)from()from()where()]*'mul(925,221):%?from()mul(981,324)%}mul(595,648)&select()/mul(742,838)when()~@from()don't()#+)(<^?}mul(554,328)} -?(mul(513,192)~~$*why()/~% mul(444,631)## mul(794,537)-#what()~how()why(310,427)(mul(781,606)< from()<mul(674,614)where()when()how(){who()when()where()where()what()mul(416,867)$!!why(54,460)^when()mul(153,262)]where()what()who()]'do()mul(179&$)%>@&mul(787,800)@how(266,66)<?~&,{mulselect()<>-{when()what()who()mul(538,246)[^*+why()where()mul(629,836)from()* [$+&]mul(77,354)%]:from()mul(260,304)@why()!/when()where()+(mul(354,804) mul(76,15)>mul@!!$]from()mul(357,50)(mul+%~]$-%%don't()(from();when()#;~when()mul(564,123)~&]<mul(985who()#mul(269,732)(mul(963:^mul(824,4)$*how()@%what() -%mul(891,926)^*how()?select()->@mul(489,580from()what(95,431)+:*- ,*'mul(745,620)how()'why()>-from()[?><mul(509,286)->!!mul(988,840))^)(]<mul(657,220)who()[;when()>]#^}mul(298,967)when():who()#who()what()mul(84,148)< ;&@^do()when()~^where()#^$mul(341,853)<+~*what(252,434)*{{mul(986,313)]&>&>mul(620,476)why()who()mul(69,875<$-&(''mul(106,787)mul(171,707)who();~when()+>;how();+mul(163,282)(];?>}* mul(571,602)/from()+/]!{+mul(372,949)$?*$(mul(921,212)@'>mul(705,437)($when()where()mul(371,384)>mul(445,760)'do()?/when()%[^mul(382,44)}&/)&$select()mul(284,899) when():%mul(554,813)$;~>mul(274,983)'')?-from()mul(668,571)when()why()mul(981,529)>where()%) #& [^mul(864,321)how()'[mul(752,285)$ mul(448,366)&why()])${;^from()who()mul(251,944)select()-&*mul(724>>)}^,select()~~}do()}))%who()#mul(652,853)";
        assert_eq!("17291584", process(input)?);
        Ok(())
    }
}
