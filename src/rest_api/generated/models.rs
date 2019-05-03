#![allow(unused_imports, unused_qualifications, unused_extern_crates)]
extern crate chrono;
extern crate uuid;


use serde::ser::Serializer;

use std::collections::HashMap;
use models;
use swagger;


/// Defines the properties of an axis. The origin and unit vectors   or defined within the universe space, but this does NOT imply   a linear conversion is possible, this only provide anchoring   of the axis as well as its absolute direction. 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Axis {
    /// Unit of the values, on this axis, for example [mm], [s],   [um]. 
    #[serde(rename = "measurement_unit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub measurement_unit: Option<String>,

    #[serde(rename = "coordinates")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub coordinates: Option<models::ValidNumbersOnThisAxis>,

    #[serde(rename = "origin")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub origin: Option<Vec<models::Point>>,

    #[serde(rename = "unit_vector")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub unit_vector: Option<Vec<models::Point>>,

}

impl Axis {
    pub fn new() -> Axis {
        Axis {
            measurement_unit: None,
            coordinates: None,
            origin: None,
            unit_vector: None,
        }
    }
}

/// Collection of Spatial Objects, stored in one or more Reference   Spaces. 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataSet {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<String>,

    /// Scale factors used to generate less precise, coarser indexes   in order to speed up queries over large volumes of the   space.  Values are expressed as powers of two, in the range [0;n].   For each scale, a whole vector providing values for each   axis MUST be provided.  Values, which are equal, and whose coordinates gets merged   are merged as well, to reduce the number of results.  Distinct values whose coordinates are merged are recorded,   thus allowing the user to move from one scale factor to   another, with a finer resolution smoothly. 
    #[serde(rename = "scales")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scales: Option<Vec<Vec<f64>>>,

}

impl DataSet {
    pub fn new() -> DataSet {
        DataSet {
            name: None,
            version: None,
            scales: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Datasets {
    #[serde(rename = "list")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub list: Option<Vec<models::DataSet>>,

}

impl Datasets {
    pub fn new() -> Datasets {
        Datasets {
            list: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Datasets1 {
    #[serde(rename = "list")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub list: Option<Vec<String>>,

}

impl Datasets1 {
    pub fn new() -> Datasets1 {
        Datasets1 {
            list: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Filters {
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filter: Option<String>,

    #[serde(rename = "ids_only")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ids_only: Option<bool>,

}

impl Filters {
    pub fn new() -> Filters {
        Filters {
            filter: None,
            ids_only: Some(false),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse200 {
    #[serde(rename = "previous")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub previous: Option<models::Space>,

    #[serde(rename = "current")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current: Option<models::Space>,

}

impl InlineResponse200 {
    pub fn new() -> InlineResponse200 {
        InlineResponse200 {
            previous: None,
            current: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2001 {
    #[serde(rename = "previous")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub previous: Option<models::DataSet>,

    #[serde(rename = "current")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current: Option<models::DataSet>,

}

impl InlineResponse2001 {
    pub fn new() -> InlineResponse2001 {
        InlineResponse2001 {
            previous: None,
            current: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2002 {
    #[serde(rename = "previous")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub previous: Option<models::SpatialObject>,

    #[serde(rename = "current")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current: Option<models::SpatialObject>,

}

impl InlineResponse2002 {
    pub fn new() -> InlineResponse2002 {
        InlineResponse2002 {
            previous: None,
            current: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartialUpdate {
    /// Identifier or name of the instance to update. 
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// Valid selector / attribute name of the instance. 
    // Note: inline enums are not fully supported by swagger-codegen
    #[serde(rename = "attribute")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attribute: Option<String>,

    /// JSON-serialized value to use to replace the value of the selected attribute. 
    #[serde(rename = "value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,

}

impl PartialUpdate {
    pub fn new() -> PartialUpdate {
        PartialUpdate {
            name: None,
            attribute: None,
            value: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartialUpdate1 {
    /// Identifier or name of the instance to update. 
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// Valid selector / attribute name of the instance. 
    // Note: inline enums are not fully supported by swagger-codegen
    #[serde(rename = "attribute")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attribute: Option<String>,

    /// JSON-serialized value to use to replace the value of the selected attribute. 
    #[serde(rename = "value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,

}

impl PartialUpdate1 {
    pub fn new() -> PartialUpdate1 {
        PartialUpdate1 {
            name: None,
            attribute: None,
            value: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartialUpdate2 {
    /// Identifier or name of the instance to update. 
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    /// Valid selector / attribute name of the instance. 
    // Note: inline enums are not fully supported by swagger-codegen
    #[serde(rename = "attribute")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attribute: Option<String>,

    /// JSON-serialized value to use to replace the value of the selected attribute. 
    #[serde(rename = "value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,

}

impl PartialUpdate2 {
    pub fn new() -> PartialUpdate2 {
        PartialUpdate2 {
            id: None,
            attribute: None,
            value: None,
        }
    }
}

/// One valid value for each axes of the reference space this point   is used in. 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Point(Vec<Number>);

impl ::std::convert::From<Vec<Number>> for Point {
    fn from(x: Vec<Number>) -> Self {
        Point(x)
    }
}

impl ::std::convert::From<Point> for Vec<Number> {
    fn from(x: Point) -> Self {
        x.0
    }
}

impl ::std::iter::FromIterator<Number> for Point {
    fn from_iter<U: IntoIterator<Item=Number>>(u: U) -> Self {
        Point(Vec::<Number>::from_iter(u))
    }
}

impl ::std::iter::IntoIterator for Point {
    type Item = Number;
    type IntoIter = ::std::vec::IntoIter<Number>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> ::std::iter::IntoIterator for &'a Point {
    type Item = &'a Number;
    type IntoIter = ::std::slice::Iter<'a, Number>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a> ::std::iter::IntoIterator for &'a mut Point {
    type Item = &'a mut Number;
    type IntoIter = ::std::slice::IterMut<'a, Number>;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl ::std::ops::Deref for Point {
    type Target = Vec<Number>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ::std::ops::DerefMut for Point {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


/// Properties tied to a shape, in other words properties valid for   the whole content of the shape. 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Properties {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    /// Label defining the kind of the spatial object. 
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

}

impl Properties {
    pub fn new() -> Properties {
        Properties {
            id: None,
            _type: None,
        }
    }
}

/// Geometric shape defined in a reference space. 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shape {
    /// Name of the shape class described by the vertices, this can   be used for specific types to reduce the number of   vertices required to define the shape. 
    // Note: inline enums are not fully supported by swagger-codegen
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    /// List of vertices composing the contour of the shape. 
    #[serde(rename = "vertices")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vertices: Option<Vec<models::Point>>,

    /// Name of a valid reference space. This is the space in which   the vertices are defined 
    #[serde(rename = "space")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub space: Option<String>,

}

impl Shape {
    pub fn new() -> Shape {
        Shape {
            _type: None,
            vertices: None,
            space: None,
        }
    }
}

/// Definition of a space, in which objects are described. 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Space {
    /// Unique Id for the space, which can also be used to generate a   link to the user documentation describing the space,   explaining the semantic meaning of the values stored, as   well as the definitions of the axes. 
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// The order of the axes matter and MUST be kept, as this is   also linked to the definition found in the documentation.  Coordinate of a point MUST always be expressed using the   same order as defined here. 
    #[serde(rename = "axes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub axes: Option<Vec<models::Axis>>,

}

impl Space {
    pub fn new() -> Space {
        Space {
            name: None,
            axes: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spaces {
    #[serde(rename = "list")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub list: Option<Vec<models::Space>>,

}

impl Spaces {
    pub fn new() -> Spaces {
        Spaces {
            list: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spaces1 {
    #[serde(rename = "list")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub list: Option<Vec<String>>,

}

impl Spaces1 {
    pub fn new() -> Spaces1 {
        Spaces1 {
            list: None,
        }
    }
}

/// Collection of positions in a space, which share a common set of   properties. 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpatialObject {
    /// List of shapes, overlapping or not, which define the whole space covered by this spatial object. 
    #[serde(rename = "shape")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub shape: Option<Vec<models::Shape>>,

    #[serde(rename = "properties")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub properties: Option<models::Properties>,

}

impl SpatialObject {
    pub fn new() -> SpatialObject {
        SpatialObject {
            shape: None,
            properties: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpatialObjects {
    #[serde(rename = "list")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub list: Option<Vec<models::SpatialObject>>,

}

impl SpatialObjects {
    pub fn new() -> SpatialObjects {
        SpatialObjects {
            list: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpatialObjects1 {
    #[serde(rename = "list")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub list: Option<Vec<String>>,

}

impl SpatialObjects1 {
    pub fn new() -> SpatialObjects1 {
        SpatialObjects1 {
            list: None,
        }
    }
}

/// Definition of the valid coordinate values which can be used   on this axis. 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidNumbersOnThisAxis {
    /// Valid numbers as defined by the usual mathematical sets,  for example N=Natural, Z=Integers, Q=Rational, R=Real. 
    // Note: inline enums are not fully supported by swagger-codegen
    #[serde(rename = "set")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub set: Option<String>,

    #[serde(rename = "minimum")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub minimum: Option<f32>,

    #[serde(rename = "maximum")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maximum: Option<f32>,

    #[serde(rename = "steps")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub steps: Option<f64>,

}

impl ValidNumbersOnThisAxis {
    pub fn new() -> ValidNumbersOnThisAxis {
        ValidNumbersOnThisAxis {
            set: None,
            minimum: None,
            maximum: None,
            steps: None,
        }
    }
}
