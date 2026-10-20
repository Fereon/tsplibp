use std::str::FromStr;

use crate::{
    datatypes::{Instance, Integer, Real},
    error::TSPLibError,
    parser::{data::parse_data_section, spec::parse_specification},
};

pub mod data;
pub mod spec;

fn split_once_newline(input: &str) -> (&str, &str) {
    let Some((s1, s2)) = input.split_once('\n') else {
        return (input.trim_end(), "");
    };
    let (s1, s2) = (s1.trim_end_matches('\r').trim_end(), s2.trim_start());
    (s1, s2)
}

fn split_once_whitespace(input: &str) -> (&str, &str) {
    let Some((s1, s2)) = input.split_once(char::is_whitespace) else {
        return (input.trim_end(), "");
    };
    let s2 = s2.trim_start();
    (s1, s2)
}

fn split_once_colon(input: &str) -> (&str, &str) {
    let Some((s1, s2)) = input.split_once(':') else {
        return (input.trim_end(), "");
    };
    let (s1, s2) = (s1.trim_end(), s2.trim_start());
    (s1, s2)
}

fn parse_integer(input: &str) -> Result<(Integer, &str), TSPLibError> {
    let (input, rest) = split_once_whitespace(input);
    let integer: Integer = input
        .parse()
        .map_err(|_| TSPLibError::InvalidIntegerFormat(input.to_string()))?;

    Ok((integer, rest))
}

fn parse_real(input: &str) -> Result<(Real, &str), TSPLibError> {
    let (input, rest) = split_once_whitespace(input);
    let real: Real = input
        .parse()
        .map_err(|_| TSPLibError::InvalidRealFormat(input.to_string()))?;

    Ok((real, rest))
}

fn parse_string(input: &str) -> (String, &str) {
    let (string, rest) = split_once_newline(input);
    (string.into(), rest)
}

impl FromStr for Instance {
    type Err = TSPLibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (specification, rest) = parse_specification(s)?;
        let (data, rest) = parse_data_section(rest, &specification)?;

        if !rest.is_empty() {
            return Err(TSPLibError::ParsingIncomplete(rest.to_string()));
        }

        Ok(Self {
            specification,
            data,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::datatypes::TwoDimensionalNode;

    #[test]
    fn test_parse_integer() {
        assert_eq!(parse_integer("123"), Ok((123, "")));
        assert_eq!(parse_integer("123 abc"), Ok((123, "abc")));
        assert_eq!(parse_integer("123\tabc"), Ok((123, "abc")));
        assert!(parse_integer("abc").is_err());
    }

    #[test]
    fn test_parse_real() {
        assert_eq!(parse_real("123.45"), Ok((123.45, "")));
        assert_eq!(parse_real("123.45 abc"), Ok((123.45, "abc")));
        assert!(parse_real("abc").is_err());
    }

    #[test]
    fn test_parse_string() {
        assert_eq!(parse_string("abc\ndef"), ("abc".into(), "def"));
        assert_eq!(parse_string("abc\r\ndef"), ("abc".into(), "def"));
        assert_eq!(parse_string("abc"), ("abc".into(), ""));
    }

    #[test]
    fn test_split_once_newline() {
        assert_eq!(split_once_newline("abc\ndef"), ("abc", "def"));
        assert_eq!(split_once_newline("abc\r\ndef"), ("abc", "def"));
        assert_eq!(split_once_newline("abc"), ("abc", ""));
        assert_eq!(split_once_newline(""), ("", ""));
        assert_eq!(split_once_newline("\n"), ("", ""));
        assert_eq!(split_once_newline("abc\n"), ("abc", ""));
    }

    #[test]
    fn test_split_once_whitespace() {
        assert_eq!(split_once_whitespace("abc def"), ("abc", "def"));
        assert_eq!(split_once_whitespace("abc   def"), ("abc", "def"));
        assert_eq!(split_once_whitespace("abc\tdef"), ("abc", "def"));
        assert_eq!(split_once_whitespace("abc\ndef"), ("abc", "def"));
        assert_eq!(split_once_whitespace("abc\r\ndef"), ("abc", "def"));
        assert_eq!(split_once_whitespace("abc \n def"), ("abc", "def"));
        assert_eq!(split_once_whitespace("abc"), ("abc", ""));
        assert_eq!(split_once_whitespace(""), ("", ""));
        assert_eq!(split_once_whitespace(" abc"), ("", "abc"));
        assert_eq!(split_once_whitespace("abc "), ("abc", ""));
    }

    #[test]
    fn test_split_once_colon() {
        assert_eq!(split_once_colon("NAME: eil51"), ("NAME", "eil51"));
        assert_eq!(split_once_colon("NAME : eil51"), ("NAME", "eil51"));
        assert_eq!(split_once_colon("NAME:  eil51"), ("NAME", "eil51"));
        assert_eq!(split_once_colon("NAME:"), ("NAME", ""));
        assert_eq!(split_once_colon(":eil51"), ("", "eil51"));
        assert_eq!(split_once_colon("NAME eil51"), ("NAME eil51", ""));
        assert_eq!(split_once_colon(""), ("", ""));
    }

    #[test]
    fn test_parse_br17() {
        let input = "NAME:  br17
TYPE: ATSP
COMMENT: 17 city problem (Repetto)
DIMENSION:  17
EDGE_WEIGHT_TYPE: EXPLICIT
EDGE_WEIGHT_FORMAT: FULL_MATRIX
EDGE_WEIGHT_SECTION
 9999    3    5   48   48    8    8    5    5    3    3    0    3    5    8    8
    5
    3 9999    3   48   48    8    8    5    5    0    0    3    0    3    8    8
    5
    5    3 9999   72   72   48   48   24   24    3    3    5    3    0   48   48
   24
   48   48   74 9999    0    6    6   12   12   48   48   48   48   74    6    6
   12
   48   48   74    0 9999    6    6   12   12   48   48   48   48   74    6    6
   12
    8    8   50    6    6 9999    0    8    8    8    8    8    8   50    0    0
    8
    8    8   50    6    6    0 9999    8    8    8    8    8    8   50    0    0
    8
    5    5   26   12   12    8    8 9999    0    5    5    5    5   26    8    8
    0
    5    5   26   12   12    8    8    0 9999    5    5    5    5   26    8    8
    0
    3    0    3   48   48    8    8    5    5 9999    0    3    0    3    8    8
    5
    3    0    3   48   48    8    8    5    5    0 9999    3    0    3    8    8
    5
    0    3    5   48   48    8    8    5    5    3    3 9999    3    5    8    8
    5
    3    0    3   48   48    8    8    5    5    0    0    3 9999    3    8    8
    5
    5    3    0   72   72   48   48   24   24    3    3    5    3 9999   48   48
   24
    8    8   50    6    6    0    0    8    8    8    8    8    8   50 9999    0
    8
    8    8   50    6    6    0    0    8    8    8    8    8    8   50    0 9999
    8
    5    5   26   12   12    8    8    0    0    5    5    5    5   26    8    8
 9999
EOF";
        let instance: Instance = input.parse().unwrap();
        assert_eq!(instance.specification.name, "br17");
        assert_eq!(instance.specification.dimension, Some(17));
        assert_eq!(instance.data.edge_weight_section.unwrap().len(), 17 * 17);
    }

    #[test]
    fn test_parse_burma14() {
        let input = "NAME: burma14
TYPE: TSP
COMMENT: 14-Staedte in Burma (Zaw Win)
DIMENSION: 14
EDGE_WEIGHT_TYPE: GEO
EDGE_WEIGHT_FORMAT: FUNCTION
DISPLAY_DATA_TYPE: COORD_DISPLAY
NODE_COORD_SECTION
   1  16.47       96.10
   2  16.47       94.44
   3  20.09       92.54
   4  22.39       93.37
   5  25.23       97.24
   6  22.00       96.05
   7  20.47       97.02
   8  17.20       96.29
   9  16.30       97.38
  10  14.05       98.12
  11  16.53       97.38
  12  21.52       95.59
  13  19.41       97.13
  14  20.09       94.55
EOF";
        let instance: Instance = input.parse().unwrap();
        assert_eq!(instance.specification.name, "burma14");
        assert_eq!(instance.specification.dimension, Some(14));
        if let Some(crate::datatypes::NodeCoordSection::TwodCoords(coords)) =
            instance.data.node_coord_section
        {
            assert_eq!(coords.len(), 14);
            assert_eq!(
                coords[0],
                TwoDimensionalNode {
                    identifier: 1,
                    x_coordinate: 16.47,
                    y_coordinate: 96.10
                }
            );
        } else {
            panic!("wrong node coord section");
        }
    }

    #[test]
    fn test_parse_gr17() {
        let input = "NAME: gr17
TYPE: TSP
COMMENT: 17-city problem (Groetschel)
DIMENSION: 17
EDGE_WEIGHT_TYPE: EXPLICIT
EDGE_WEIGHT_FORMAT: LOWER_DIAG_ROW
EDGE_WEIGHT_SECTION
 0 633 0 257 390 0 91 661 228 0 412 227
 169 383 0 150 488 112 120 267 0 80 572 196
 77 351 63 0 134 530 154 105 309 34 29 0
 259 555 372 175 338 264 232 249 0 505 289 262
 476 196 360 444 402 495 0 353 282 110 324 61
 208 292 250 352 154 0 324 638 437 240 421 329
 297 314 95 578 435 0 70 567 191 27 346 83
 47 68 189 439 287 254 0 211 466 74 182 243
 105 150 108 326 336 184 391 145 0 268 420 53
 239 199 123 207 165 383 240 140 448 202 57 0
 246 745 472 237 528 364 332 349 202 685 542 157
 289 426 483 0 121 518 142 84 297 35 29 36
 236 390 238 301 55 96 153 336 0
EOF";
        let instance: Instance = input.parse().unwrap();
        assert_eq!(instance.specification.name, "gr17");
        assert_eq!(
            instance.data.edge_weight_section.unwrap().len(),
            17 * (17 + 1) / 2
        );
    }

    #[test]
    fn test_parse_eil7() {
        let input = "NAME : eil7
COMMENT : (Eilon et al.)
TYPE : CVRP
DIMENSION : 7
EDGE_WEIGHT_TYPE : EXPLICIT
EDGE_WEIGHT_FORMAT: LOWER_COL
DISPLAY_DATA_TYPE: NO_DISPLAY
CAPACITY : 3
EDGE_WEIGHT_SECTION
    10    20    25    25    20    10    12    20    25    30
    20    10    11    22    30     2    11    25    10    20
    12
DEMAND_SECTION
1 0
2 1
3 1
4 1
5 1
6 1
7 1
DEPOT_SECTION
1
-1
EOF";
        let instance: Instance = input.parse().unwrap();
        assert_eq!(instance.specification.name, "eil7");
        assert_eq!(instance.specification.capacity, Some(3));
        assert_eq!(
            instance.data.edge_weight_section.unwrap().len(),
            7 * (7 - 1) / 2
        );
        assert_eq!(instance.data.demand_section.unwrap().len(), 7);
        assert_eq!(instance.data.depot_section.unwrap(), vec![1]);
    }

    #[test]
    fn test_parse_sop_esc07() {
        let input = "NAME: ESC07.sop
TYPE: SOP
COMMENT: Received by Norbert Ascheuer / Laureano Escudero
DIMENSION: 9
EDGE_WEIGHT_TYPE: EXPLICIT
EDGE_WEIGHT_FORMAT: FULL_MATRIX
EDGE_WEIGHT_SECTION
9
    0    0    0    0    0    0    0    0 1000000
   -1    0  100  200   75    0  300  100    0
   -1  400    0  500  325  400  600    0    0
   -1  700  800    0  550  700  900  800    0
   -1   -1  250  225    0  275  525  250    0
   -1   -1  100  200   -1    0   -1   -1    0
   -1   -1 1100 1200 1075 1000    0 1100    0
   -1   -1 -1    -1   -1   -1   -1    0    0
   -1   -1 -1    -1   -1   -1   -1   -1    0
EOF";
        let instance: Instance = input.parse().unwrap();
        assert_eq!(instance.specification.name, "ESC07.sop");
        assert_eq!(instance.data.edge_weight_section.unwrap().len(), 9 * 9);
    }
}
