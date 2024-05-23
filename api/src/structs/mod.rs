// Desc: Structs for the sorting and pathfinding endpoints
use serde::{Deserialize, Serialize};

/***
* Desc: Struct for the input of the sorting endpoint
* Fields:
* - sort: String
*/
#[derive(Deserialize, Serialize)]
pub struct SortingInput {
    pub sort: String,
    pub arr: Vec<i32>,
}

/**
* Desc: Struct for the output of the sorting endpoint
* Fields:
* - sort: String
*/
#[derive(Deserialize, Serialize)]
pub struct SortingOutput {
    pub sort: String,
    pub cycles: Vec<Cycle>,
}

/***
 * Desc: Struct for the input of the pathfinding endpoint
 * Fields:
 * - message: String
 */
#[derive(Deserialize, Serialize)]
pub struct PathInput {
    pub message: String,
}

/***
 * Desc: Struct for the output of the pathfinding endpoint
 * Fields:
 * - message: String
 */
#[derive(Deserialize, Serialize)]
pub struct PathOutput {
    pub message: String,
}

/***
 * Desc: Enum for the sorting algorithm types
 * Variants:
 * - BubbleSort
 *
 * The enum defines the different sorting algorithm types that can be used.
 * The variants represent the different sorting algorithms that can be used.
 * The as_type function converts a string to the corresponding SortType variant.
 */
pub enum SortType {
    BubbleSort,
}

/***
 * Desc: Implementation of the SortType enum
 * The implementation block defines associated functions for the SortType enum.
 * The as_type function converts a string to the corresponding SortType variant.
 */
impl SortType {
    pub fn as_type(string: &str) -> Option<SortType> {
        match string {
            "BubbleSort" => Some(SortType::BubbleSort),
            _ => None,
        }
    }
}

/***
 * Desc: Object that represents a cycle in the sorting algorithm.
 * Fields:
 * - elements: Vec<ArrayElement>
 */
#[derive(Deserialize, Serialize)]
pub struct Cycle {
    pub cycle: Vec<Step>,
}

/***
 * Desc: Object that represents a step in the cycle in the sorting algorithm.
 * Fields:
 * - elements: Vec<ArrayElement>
 */
#[derive(Deserialize, Serialize)]
pub struct Step {
    pub swapped: bool,
    pub compared: Vec<(usize, usize)>,
}
