use crate::{
    shapes::{Group, Triangle},
    Point, Vector,
};

struct ObjParser {
    ignored_lines: usize,
    vertices: Vec<Point>,
    normals: Vec<Vector>,
    default_group: Group,
}

/// Build objects by parsing a Wavefront OBJ file
pub struct ObjFile {}

enum GroupType {
    Parent,
    Child(Box<Group>),
}

impl ObjFile {
    /// Parse a Wavefront OBJ string returning a [`Group`] object with all of the
    /// triangles and polygons in the `buffer`.
    pub fn parse(buffer: &str) -> Group {
        let parser = ObjFile::parse_obj_file(buffer);
        parser.default_group
    }

    fn parse_obj_file(buffer: &str) -> ObjParser {
        let mut parser = ObjParser {
            ignored_lines: 0,
            vertices: Vec::new(),
            normals: Vec::new(),
            default_group: Group::new(),
        };

        let mut group = GroupType::Parent;

        for line in buffer.lines() {
            let mut line_iter = line.split_whitespace();
            if let Some(token) = line_iter.next() {
                match token {
                    "v" => {
                        let x: f64 = line_iter.next().unwrap().parse().unwrap();
                        let y: f64 = line_iter.next().unwrap().parse().unwrap();
                        let z: f64 = line_iter.next().unwrap().parse().unwrap();
                        parser.vertices.push(Point::new(x, y, z));
                    }
                    "vn" => {
                        let x: f64 = line_iter.next().unwrap().parse().unwrap();
                        let y: f64 = line_iter.next().unwrap().parse().unwrap();
                        let z: f64 = line_iter.next().unwrap().parse().unwrap();
                        parser.normals.push(Vector::new(x, y, z));
                    }
                    "f" => {
                        ObjFile::parse_faces(&mut parser, &mut line_iter, &mut group);
                    }
                    "g" => match group {
                        GroupType::Parent => {
                            let mut child_group = Group::new();
                            child_group.inherit_material = true;
                            group = GroupType::Child(Box::new(child_group));
                        }
                        GroupType::Child(g) => {
                            parser.default_group.add_object(g);
                            let mut child_group = Group::new();
                            child_group.inherit_material = true;
                            group = GroupType::Child(Box::new(child_group));
                        }
                    },
                    _ => {
                        parser.ignored_lines += 1;
                    }
                }
            }
        }

        if let GroupType::Child(g) = group {
            parser.default_group.add_object(g);
        }

        parser
    }

    fn parse_faces(
        parser: &mut ObjParser,
        line_iter: &mut std::str::SplitWhitespace,
        group: &mut GroupType,
    ) {
        let mut vg: Vec<(i32, i32)> = Vec::new();
        let mut has_vn = false;
        for v in line_iter.by_ref() {
            if v.contains('/') {
                has_vn = true;
                let v_vt_vn: Vec<&str> = v.split('/').collect();
                let vi: i32 = v_vt_vn[0].parse().unwrap();
                let vni: i32 = v_vt_vn[2].parse().unwrap();
                vg.push((vi - 1, vni - 1));
            } else {
                let vi: i32 = v.parse().unwrap();
                vg.push((vi - 1, 0));
            }
        }
        for index in 1..vg.len() - 1 {
            if has_vn {
                let p1 = parser.vertices[vg[0].0 as usize];
                let p2 = parser.vertices[vg[index].0 as usize];
                let p3 = parser.vertices[vg[index + 1].0 as usize];

                let n1 = parser.normals[vg[0].1 as usize];
                let n2 = parser.normals[vg[index].1 as usize];
                let n3 = parser.normals[vg[index + 1].1 as usize];

                let tri = Triangle::smooth_triangle(p1, p2, p3, n1, n2, n3);
                match group {
                    GroupType::Parent => parser.default_group.add_object(Box::new(tri)),
                    GroupType::Child(g) => g.add_object(Box::new(tri)),
                }
            } else {
                let p1 = parser.vertices[vg[0].0 as usize];
                let p2 = parser.vertices[vg[index].0 as usize];
                let p3 = parser.vertices[vg[index + 1].0 as usize];

                let tri = Triangle::new(p1, p2, p3);
                match group {
                    GroupType::Parent => parser.default_group.add_object(Box::new(tri)),
                    GroupType::Child(g) => g.add_object(Box::new(tri)),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::Triangle;

    // Chapter 15 Triangles
    // Page 213
    #[test]
    fn ignoring_unrecognized_lines() {
        let gibberish = "There was a young lady named Bright
who traveled much faster than light.
She set out one day
in a relative way,
and came back the previous night.";
        let parser = ObjFile::parse_obj_file(gibberish);

        assert_eq!(parser.ignored_lines, 5);
    }

    // Chapter 15 Triangles
    // Page 213
    #[test]
    fn vertex_records() {
        let file = "v -1 1 0
v -1.000000 0.50000 0.0000
v 1 0 0
v 1 1 0";
        let parser = ObjFile::parse_obj_file(file);

        assert_eq!(parser.ignored_lines, 0);
        assert_eq!(parser.vertices.len(), 4);
        assert_eq!(parser.vertices[0], Point::new(-1.0, 1.0, 0.0));
        assert_eq!(parser.vertices[1], Point::new(-1.0, 0.5, 0.0));
        assert_eq!(parser.vertices[2], Point::new(1.0, 0.0, 0.0));
        assert_eq!(parser.vertices[3], Point::new(1.0, 1.0, 0.0));
    }

    // Chapter 15 Triangles
    // Page 215
    #[test]
    fn parsing_triangle_faces() {
        let file = "
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0

f 1 2 3
f 1 3 4";
        let parser = ObjFile::parse_obj_file(file);
        let g = &parser.default_group;
        let t1 = g.get_object(0).unwrap();
        let t1 = t1.as_any().unwrap().downcast_ref::<Triangle>().unwrap();
        let t2 = g.get_object(1).unwrap();
        let t2 = t2.as_any().unwrap().downcast_ref::<Triangle>().unwrap();

        assert_eq!(t1.p1, parser.vertices[0]);
        assert_eq!(t1.p2, parser.vertices[1]);
        assert_eq!(t1.p3, parser.vertices[2]);
        assert_eq!(t2.p1, parser.vertices[0]);
        assert_eq!(t2.p2, parser.vertices[2]);
        assert_eq!(t2.p3, parser.vertices[3]);
    }

    // Chapter 15 Triangles
    // Page 214 and 215
    #[test]
    fn triangulating_polygons() {
        let file = "
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
v 0 2 0

f 1 2 3 4 5";
        let parser = ObjFile::parse_obj_file(file);
        let g = &parser.default_group;
        let t1 = g.get_object(0).unwrap();
        let t1 = t1.as_any().unwrap().downcast_ref::<Triangle>().unwrap();
        let t2 = g.get_object(1).unwrap();
        let t2 = t2.as_any().unwrap().downcast_ref::<Triangle>().unwrap();
        let t3 = g.get_object(2).unwrap();
        let t3 = t3.as_any().unwrap().downcast_ref::<Triangle>().unwrap();

        assert_eq!(t1.p1, parser.vertices[0]);
        assert_eq!(t1.p2, parser.vertices[1]);
        assert_eq!(t1.p3, parser.vertices[2]);
        assert_eq!(t2.p1, parser.vertices[0]);
        assert_eq!(t2.p2, parser.vertices[2]);
        assert_eq!(t2.p3, parser.vertices[3]);
        assert_eq!(t3.p1, parser.vertices[0]);
        assert_eq!(t3.p2, parser.vertices[3]);
        assert_eq!(t3.p3, parser.vertices[4]);
    }

    // Chapter 15 Triangles
    // Page 215
    #[test]
    fn triangles_in_groups() {
        let file = "
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0

g FirstGroup
f 1 2 3
g SecondGroup
f 1 3 4";
        let parser = ObjFile::parse_obj_file(file);
        let g = &parser.default_group;

        let g1 = g.get_object(0).unwrap();
        let g1 = g1.as_any().unwrap().downcast_ref::<Group>().unwrap();

        let g2 = g.get_object(1).unwrap();
        let g2 = g2.as_any().unwrap().downcast_ref::<Group>().unwrap();

        let t1 = g1.get_object(0).unwrap();
        let t1 = t1.as_any().unwrap().downcast_ref::<Triangle>().unwrap();

        let t2 = g2.get_object(0).unwrap();
        let t2 = t2.as_any().unwrap().downcast_ref::<Triangle>().unwrap();

        assert_eq!(t1.p1, parser.vertices[0]);
        assert_eq!(t1.p2, parser.vertices[1]);
        assert_eq!(t1.p3, parser.vertices[2]);
        assert_eq!(t2.p1, parser.vertices[0]);
        assert_eq!(t2.p2, parser.vertices[2]);
        assert_eq!(t2.p3, parser.vertices[3]);
    }

    // Chapter 15 Triangles
    // Page 223 & 224
    #[test]
    fn vertex_normal_records() {
        let file = "vn 0 0 1
vn 0.707 0 -0.707
vn 1 2 3";
        let parser = ObjFile::parse_obj_file(file);

        assert_eq!(parser.normals[0], Vector::new(0.0, 0.0, 1.0));
        assert_eq!(parser.normals[1], Vector::new(0.707, 0.0, -0.707));
        assert_eq!(parser.normals[2], Vector::new(1.0, 2.0, 3.0));
    }

    // Chapter 15 Triangles
    // Page 224
    #[test]
    fn faces_with_normals() {
        let file = "
v 0 1 0
v -1 0 0
v 1 0 0

vn -1 0 0
vn 1 0 0
vn 0 1 0

f 1//3 2//1 3//2
f 1/0/3 2/102/1 3/14/2
";
        let parser = ObjFile::parse_obj_file(file);
        let g = &parser.default_group;
        let t1 = g.get_object(0).unwrap();
        let t1 = t1.as_any().unwrap().downcast_ref::<Triangle>().unwrap();
        let t2 = g.get_object(1).unwrap();
        let t2 = t2.as_any().unwrap().downcast_ref::<Triangle>().unwrap();

        assert_eq!(t1.p1, parser.vertices[0]);
        assert_eq!(t1.p2, parser.vertices[1]);
        assert_eq!(t1.p3, parser.vertices[2]);
        assert_eq!(t1.n1.unwrap(), parser.normals[2]);
        assert_eq!(t1.n2.unwrap(), parser.normals[0]);
        assert_eq!(t1.n3.unwrap(), parser.normals[1]);

        assert_eq!(t2.p1, t1.p1);
        assert_eq!(t2.p2, t1.p2);
        assert_eq!(t2.p3, t1.p3);
        assert_eq!(t2.n1.unwrap(), t1.n1.unwrap());
        assert_eq!(t2.n2.unwrap(), t1.n2.unwrap());
        assert_eq!(t2.n3.unwrap(), t1.n3.unwrap());
    }
}
