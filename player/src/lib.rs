use wad::WadFile;

pub struct Player {}

#[derive(Debug, Clone)]
pub struct BspNode {
    pub x: i16,
    pub y: i16,
    pub dx: i16,
    pub dy: i16,
    pub bbox_right: [i16; 4],
    pub bbox_left: [i16; 4],
    pub right_child: u16,
    pub left_child: u16,
}

#[derive(Debug, Clone)]
pub struct BspTree {
    pub nodes: Vec<BspNode>,
    pub subsectors: Vec<Subsector>,
    pub segs: Vec<Seg>,
}

#[derive(Debug, Clone)]
pub struct Subsector {
    pub seg_count: u16,
    pub first_seg: u16,
}

#[derive(Debug, Clone)]
pub struct Seg {
    pub start_vertex: u16,
    pub end_vertex: u16,
    pub angle: u16,
    pub linedef: u16,
    pub direction: u16,
    pub offset: u16,
}

impl BspTree {
    pub fn load_from_wad(wad: &WadFile, map_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let map_index = wad.lumps.iter().position(|lump| lump.name == map_name)
            .ok_or("Map not found")?;

        let nodes = Self::parse_nodes(&wad.lumps[map_index + 7].data)?;
        let subsectors = Self::parse_subsectors(&wad.lumps[map_index + 6].data)?;
        let segs = Self::parse_segs(&wad.lumps[map_index + 5].data)?;

        Ok(BspTree { nodes, subsectors, segs })
    }

    fn parse_nodes(data: &[u8]) -> Result<Vec<BspNode>, Box<dyn std::error::Error>> {
        let mut cursor = std::io::Cursor::new(data);
        let mut nodes = Vec::new();

        while cursor.position() < data.len() as u64 {
            use byteorder::{LittleEndian, ReadBytesExt};

            let x = cursor.read_i16::<LittleEndian>()?;
            let y = cursor.read_i16::<LittleEndian>()?;
            let dx = cursor.read_i16::<LittleEndian>()?;
            let dy = cursor.read_i16::<LittleEndian>()?;

            let mut bbox_right = [0i16; 4];
            let mut bbox_left = [0i16; 4];

            for i in 0..4 {
                bbox_right[i] = cursor.read_i16::<LittleEndian>()?;
            }
            for i in 0..4 {
                bbox_left[i] = cursor.read_i16::<LittleEndian>()?;
            }

            let right_child = cursor.read_u16::<LittleEndian>()?;
            let left_child = cursor.read_u16::<LittleEndian>()?;

            nodes.push(BspNode {
                x, y, dx, dy,
                bbox_right,
                bbox_left,
                right_child,
                left_child,
            });
        }

        Ok(nodes)
    }

    pub fn traverse_bsp(&self, player_x: f64, player_y: f64, node_index: u16) -> Vec<u16> {
        if node_index & 0x8000 != 0 {
            return vec![node_index & 0x7FFF];
        }

        let node = &self.nodes[node_index as usize];
        let side = self.point_on_side(player_x, player_y, node);

        let mut visible_subsectors = Vec::new();

        if side <= 0 {
            visible_subsectors.extend(self.traverse_bsp(player_x, player_y, node.left_child));
            if self.bbox_visible(player_x, player_y, &node.bbox_right) {
                visible_subsectors.extend(self.traverse_bsp(player_x, player_y, node.right_child));
            }
        } else {
            visible_subsectors.extend(self.traverse_bsp(player_x, player_y, node.right_child));
            if self.bbox_visible(player_x, player_y, &node.bbox_left) {
                visible_subsectors.extend(self.traverse_bsp(player_x, player_y, node.left_child));
            }
        }

        visible_subsectors
    }

    fn point_on_side(&self, x: f64, y: f64, node: &BspNode) -> i32 {
        let dx = x - node.x as f64;
        let dy = y - node.y as f64;

        let cross_product = dx * node.dy as f64 - dy * node.dx as f64;

        if cross_product > 0.0 { 1 } else { -1 }
    }

    fn bbox_visible(&self, player_x: f64, player_y: f64, bbox: &[i16; 4]) -> bool {
        let distance = ((bbox[2] as f64 - player_x).powi(2) + (bbox[3] as f64 - player_y).powi(2)).sqrt();
        distance < 1000.0
    }

    fn parse_subsectors(data: &[u8]) -> Result<Vec<Subsector>, Box<dyn std::error::Error>> {
        let mut cursor = std::io::Cursor::new(data);
        let mut subsectors = Vec::new();

        while cursor.position() < data.len() as u64 {
            use byteorder::{LittleEndian, ReadBytesExt};

            let seg_count = cursor.read_u16::<LittleEndian>()?;
            let first_seg = cursor.read_u16::<LittleEndian>()?;

            subsectors.push(Subsector { seg_count, first_seg });
        }

        Ok(subsectors)
    }

    fn parse_segs(data: &[u8]) -> Result<Vec<Seg>, Box<dyn std::error::Error>> {
        let mut cursor = std::io::Cursor::new(data);
        let mut segs = Vec::new();

        while cursor.position() < data.len() as u64 {
            use byteorder::{LittleEndian, ReadBytesExt};

            let start_vertex = cursor.read_u16::<LittleEndian>()?;
            let end_vertex = cursor.read_u16::<LittleEndian>()?;
            let angle = cursor.read_u16::<LittleEndian>()?;
            let linedef = cursor.read_u16::<LittleEndian>()?;
            let direction = cursor.read_u16::<LittleEndian>()?;
            let offset = cursor.read_u16::<LittleEndian>()?;

            segs.push(Seg {
                start_vertex,
                end_vertex,
                angle,
                linedef,
                direction,
                offset,
            });
        }

        Ok(segs)
    }
}
