use std::io::prelude::*;
use std::io::BufReader;
use std::io::Cursor;
use std::io::SeekFrom;
use std::fs::File;
use std::env;
use std::path::Path;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};

/*struct Triangle {
    a: u16,
    b: u16,
    c: u16,
}

#[derive(Copy, Clone)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Copy, Clone)]
struct Normal {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Copy, Clone)]
struct TextureCoordinate {
    u: f32,
    v: f32,
}*/

const HEADER_SIZE_BYTES: u32 = 6;
const MAIN_CHUNK: u16 = 0x4d4d;
const EDITOR_CHUNK: u16 = 0x3d3d;
const OBJECT_CHUNK: u16 = 0x4000;
const POLYGON_CHUNK: u16 = 0x4100;
const VERTEX_CHUNK: u16 = 0x4110;
const FACES_CHUNK: u16 = 0x4120;
const UV_CHUNK: u16 = 0x4140;
const SMOOTHING_CHUNK: u16 = 0x4150;
const MATRIX_CHUNK: u16 = 0x4160;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32;2],
    //normal: [f32;3],
    uv: [f32;2],
}

implement_vertex!(Vertex, position, uv);

#[allow(dead_code)]
pub struct Model {
    pub number_of_verts: u16,
    pub number_of_indices: u32,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl Model {
    pub fn new() -> Model {
        Model {
            number_of_verts: 0,
            number_of_indices: 0,
            vertices: Vec::new(),
            indices: Vec::new()
        }
    }

    /// https://en.wikipedia.org/wiki/.3ds
    pub fn load(&mut self, s: &str)
    {
	    let vertex1 =
		    Vertex { position: [-0.5, -0.5], uv: [0.0, 0.0] };
	    let vertex2 =
		    Vertex { position: [ -0.5,  0.5], uv: [0.0, 1.0] };
	    let vertex3 = 
		    Vertex { position: [ 0.5, -0.5], uv: [1.0, 0.0] };
	    let vertex4 = 
		    Vertex { position: [ 0.5, 0.5], uv: [1.0, 1.0] };
	    self.vertices = vec![vertex1, vertex2, vertex3, vertex4];
    
        
        let mut exe_path = env::current_exe().unwrap();
        exe_path.pop();
        let path = exe_path.join(s);

        println!("{}", path.display());
        let file = match File::open(path) {
            Ok(file) => file,
            Err(..) => panic!("model failed to load!"),
        };

        let mut reader = BufReader::new(&file);
        let data = file.metadata().unwrap();
        let size = data.len() as u32;
        let mut pos: u32 = 0;
        println!("File size: {}", size);

        while pos < size {
            println!("Reading HEADER at pos: {}", pos);
            let chunk_id = reader.read_u16::<LittleEndian>().unwrap();
            let chunk_size = reader.read_u32::<LittleEndian>().unwrap();

            pos += HEADER_SIZE_BYTES;
            println!("Chunk: {:#X}, {} bytes", chunk_id, chunk_size);

            match chunk_id {
                MAIN_CHUNK => println!("MAIN CHUNK"),
                EDITOR_CHUNK => println!("EDITOR CHUNK"),
                OBJECT_CHUNK => {
                    println!("OBJECT CHUNK");
                    let mut name = String::with_capacity(20);
                    let mut buffer = [0; 20];
                    reader.read_exact(&mut buffer);
                    let mut i: u32 = 0;
                    let zero: u8 = 0;
                    for byte in &buffer {
                       if byte == &zero {
                           break;
                       }
                        name.push(*byte as char);
                        i += 1;
                    }
                    println!("Name: {} (length = {})", name, i);
                    pos += i + 1;
                    reader.seek(SeekFrom::Start(pos as u64));
                },
                POLYGON_CHUNK => {
                    println!("POLYGON_CHUNK");
                },
                VERTEX_CHUNK => {
                    self.number_of_verts = reader.read_u16::<LittleEndian>().unwrap();
                    let v_count = self.number_of_verts as usize;
                    //self.vertices = Vec::with_capacity(v_count);
                    for n in 0..self.number_of_verts {
                        let mut buffer = [0; 12];
                        reader.read_exact(&mut buffer);
                        let x = LittleEndian::read_f32(&mut buffer);
                        let y = LittleEndian::read_f32(&mut buffer);
                        let z = LittleEndian::read_f32(&mut buffer);
                        println!("Vert {}: ({},{},{})", n, x, y, z);
                    }
                    println!("VERTEX_CHUNK: Vert count: {}", self.number_of_verts);
                    // TODO: read in vertices
                    pos += (chunk_size - HEADER_SIZE_BYTES);
                    reader.seek(SeekFrom::Start(pos as u64));
                },
                FACES_CHUNK => {
                    let faces = reader.read_u16::<LittleEndian>().unwrap();
                    println!("FACES_CHUNK: Face count: {}", faces);
                    self.number_of_indices = faces as u32 * 3;
                    // TODO: read in faces
                    pos += (chunk_size - HEADER_SIZE_BYTES);
                    reader.seek(SeekFrom::Start(pos as u64));
                },
                UV_CHUNK => {
                    let vert_count = reader.read_u16::<LittleEndian>().unwrap();
                    println!("UV_CHUNK: Vert count: {}", vert_count);
                    // TODO: read in tex-coords
                    pos += (chunk_size - HEADER_SIZE_BYTES);
                    reader.seek(SeekFrom::Start(pos as u64));
                },
                MATRIX_CHUNK => {
                    println!("MATRIX_CHUNK: Ignoring and SKIPPING TO POS: {}", pos);
                    pos += (chunk_size - HEADER_SIZE_BYTES);
                    reader.seek(SeekFrom::Start(pos as u64));
                },
                _ => {
                    pos += (chunk_size - HEADER_SIZE_BYTES);
                    println!("UNKNOWN CHUNK: SKIPPING TO POS: {}", pos);
                    reader.seek(SeekFrom::Start(pos as u64));
                }
            }
        }
        
    }
}