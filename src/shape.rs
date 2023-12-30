use crate::Vertex;

pub fn get_vertices(sides: usize) -> Vec<Vertex> {
    let theta = 2.0 * std::f32::consts::PI / sides as f32;
    let mut vertices: Vec<Vertex> = (0..sides)
        .map(|i| {
            let (sin, cos) = (i as f32 * theta).sin_cos();
            Vertex {
                position: [sin, cos, 0.0],
                color: [1.0, 1.0, 1.0],
            }
        })
        .collect();
    vertices.push(Vertex {
        position: [0.0, 0.0, 0.0],
        color: [1.0, 1.0, 1.0],
    });

    vertices
}

pub fn get_indices(vertices: &Vec<Vertex>) -> Vec<u16> {
    let origin = vertices.len() as u16 - 1;
    let mut temp: Vec<u16> = Vec::from_iter(0..((vertices.len() - 1) as u16));
    temp.push(0);
    let indices: Vec<u16> =
        temp.windows(2)
            .map(|x| vec![x[0], origin, x[1]])
            .fold(vec![], |mut acc, x| {
                acc.extend_from_slice(&x[..]);
                acc
            });
    indices
}
