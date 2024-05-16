
use bevy::{
    prelude::*, 
    render::texture::Image
};
use std::{
    error::Error,
    path::PathBuf,
    borrow::Cow,
};
use dicom::{
    core::{value::ConvertValueError, Tag},
    object::open_file,
};

pub struct DragNDropPlugin;

impl Plugin for DragNDropPlugin {
    fn build(&self, app: &mut App) 
    {
        app
        .add_systems(Update, file_drop);
    }
}

fn file_drop(
    mut events: EventReader<FileDragAndDrop>,
) {
    for event in events.read() {
        println!("{:?}", event);
        if let FileDragAndDrop::DroppedFile { window, path_buf } = event 
        {
            load_file(path_buf);
        }
    }
}

fn load_file(
    path_buf: &PathBuf,
) {
    let path = path_buf.as_path();
    let filename = path.file_name().unwrap().to_str().unwrap();
    let extension = path.extension().unwrap().to_str().unwrap();
    
    match extension {
        "zip" => extract_zip(path_buf),
        "dcm" => extract_dicom(path_buf),
        _ => println!("Dropped file with path: {:?}, filename: {:?}, extension: {:?}", path, filename, extension),
    }
}

fn extract_zip(
    path_buf: &PathBuf, 
) {
    //extract path, filename and extension from path_buf
    let path = path_buf.as_path();
    let filename = path.file_name().unwrap().to_str().unwrap();
    let extension = path.extension().unwrap().to_str().unwrap();
    
    if extension != "zip" {
        return;
    }
    
    println!("Dropped zip file with path: {:?}, filename: {:?}, extension: {:?}", path, filename, extension);

    //extract zip file
    //TODO: extract dicoms from zip file
}

fn extract_dicom(
    path_buf: &PathBuf,
) {
    //extract path, filename and extension from path_buf
    let path = path_buf.as_path();
    let filename = path.file_name().unwrap().to_str().unwrap();
    let extension = path.extension().unwrap().to_str().unwrap();
    
    if extension != "dcm" {
        return;
    }
    
    println!("Dropped dicom file with path: {:?}, filename: {:?}, extension: {:?}", path, filename, extension);

    //load dicom file
    if let Ok(pixel_data_bytes) = open_dicom_file(path_buf) {
        println!("Pixel Data Bytes: {:?}", pixel_data_bytes);

        let img = new Image();

    }
}

fn open_dicom_file(
    path_buf: &PathBuf,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let obj = open_file(path_buf)?;
    let pixel_data_bytes = obj.element(Tag(0x7FE0, 0x0010))?.to_bytes()?;

    Ok(pixel_data_bytes.to_vec())
}