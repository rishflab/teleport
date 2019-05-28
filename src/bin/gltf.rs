extern crate blackhole;

use blackhole::asset;

use std::{fs, io, path};

use std::fs::File;
use std::boxed::Box;
use std::error::Error as StdError;
use path::Path;
use gltf;
use gltf::buffer::Source;
use gltf::json;
use gltf::buffer::{View};
use gltf::Accessor;
use gltf::accessor::DataType;
use image::load;

use std::iter::FromIterator;


fn run(path: &str) -> Result<gltf::Gltf, Box<StdError>> {
    let file = fs::File::open(&path)?;
    let reader = io::BufReader::new(file);
    let gltf = gltf::Gltf::from_reader(reader)?;
    //println!("{:#?}", asset);
    Ok(gltf)
}

fn load_from_source(source: &Source) -> Vec<u8> {

    use std::io::Read;

    let mut v = Vec::new();
    match source {
        Source::Uri(path) => {
            let mut file = File::open(&path).expect("Couldn't find file");
            file.read_to_end(&mut v);
        },
        Source::Bin => {
            ()
        },
    };

    v

}

fn extract_data(buffer_data: &Vec<Vec<u8>>, views: &Vec<View>, accessor: &Accessor) -> Vec<u8> {

    let mut bd = buffer_data.clone();
    let mut buffer = bd.remove(accessor.view().buffer().index());

    match accessor.data_type() {
//        DataType::U16 => {
//            let offset = accessor.view().offset() + accessor.offset();
//            let mut data: Vec<u8> = buffer[offset..(offset + accessor.size() * accessor.count())].to_vec();
//            println!("{:?}", data.len());
//            println!("{:?}", data);
//            let bytes = bytes_to_u32(data, 2);
//            println!("{:?}", bytes);
//            u32_to_bytes(bytes)
//        },
        _ => {
            let offset = accessor.view().offset() + accessor.offset();
            let mut data: Vec<u8> = buffer[offset..(offset + accessor.size() * accessor.count())].to_vec();
            println!("{:?}", data.len());
            println!("{:?}", data);
            data
        },
    }

}

fn bytes_to_u32(bytes: Vec<u8>, step: usize) -> Vec<u32> {
    use byteorder::{LittleEndian, ByteOrder};


    let mut result: Vec<u16> = Vec::new();

    for i in (0..bytes.len()).step_by(step) {
        result.push(LittleEndian::read_u16(&bytes[i..]));
    }

    result.into_iter().map(|value|{
        value as u32
    }).collect()

}

fn u32_to_bytes(vec: Vec<u32>) -> Vec<u8> {
    use byteorder::{LittleEndian, ByteOrder};

    let mut result = Vec::new();
    vec.into_iter().for_each(|value|{
        let mut buf: [u8; 4] = [0; 4];
        LittleEndian::write_u32(&mut buf, value);
        result.append(&mut buf.to_vec());
    });
    result
}

pub struct MeshData {
    indices: Vec<u8>,
    positions: Vec<u8>,
    normals: Vec<u8>,
}

pub fn mesh_data_from_gltf(gltf: &gltf::Gltf) -> MeshData {

    let mut sources = Vec::new();

    for b in gltf.buffers() {
        println!("Buffer {:?} info: ", b.name());
        println!("{:?}", b.index());
        println!("{:?}", b.length());
        println!("{:?}", b.source());
        sources.push(b.source());

    }

    let views: Vec<View> = gltf.views().collect();

    let buffers: Vec<Vec<u8>> = sources
        .iter()
        .map(|source|{
            load_from_source(&Source::Uri("assets/Box0.bin"))
        }).collect();

    println!("buffer count: {:?}", buffers.len());
    println!("{:?}", buffers);



    let mut accessor_data: Vec<Vec<u8>> = gltf.accessors().map(|a|{
        println!("Accessor {:?} info:", a.index());
        println!("offset: {:?}", a.offset());
        println!("count: {:?}", a.count());
        println!("dimension: {:?}", a.dimensions());
        println!("size: {:?}", a.size());
        println!("data type: {:?}", a.data_type());
        let buf = extract_data(&buffers, &views, &a);
        buf

    }).collect();


    let (indices, positions, normals) = {

        let mut positions = Vec::new();
        let mut indices : Vec<u8> = Vec::new();
        let mut normals = Vec::new();

        for a in gltf.meshes() {
            println!("Mesh name: {:?}", a.name());
            println!("Mesh index: {:?}", a.index());

            for p in a.primitives() {
                p.attributes().for_each(|(s, a)| {
                    println!("accessor semantic: {:?}", s);
                    println!("accessor index: {:?}", a.index());
                    println!("accessor offset: {:?}", a.offset());
                    match s {
                        gltf::json::mesh::Semantic::Positions => {
                            positions = accessor_data[a.index()].clone();
                        },
                        gltf::json::mesh::Semantic::Normals => {
                            normals = accessor_data[a.index()].clone();
                        },
                        _ => (),
                    }

                });

                println!("primitive index: {:?}", p.indices().unwrap().index());
                indices = accessor_data[p.indices().unwrap().index()].clone();
            }
        }

        (indices, positions, normals)


    };
    println!("indices: {:?}", indices.len());
    println!("positions {:?}", positions.len());
    println!("normals {:?}", normals.len());

    MeshData {
        indices,
        positions,
        normals,
    }

}

fn main() {

    let gltf = run("assets/Box.gltf").expect("runtime error");
    mesh_data_from_gltf(&gltf);

}