
use super::Point2D;
use crate::error::{FaFsError, FaFsResult};

#[derive(Debug, Clone, Copy)]
pub struct HilbertCurve {
    pub order: u32,
}

impl HilbertCurve {
    pub fn new(order: u32) -> FaFsResult<Self> {
        if order == 0 || order > 16 {
            return Err(FaFsError::FractalError(
                "Hilbert order must be between 1 and 16".into(),
            ));
        }

        Ok(Self { order })
    }

    pub fn encode(&self, point: Point2D) -> FaFsResult<u64> {
        let size = 1u32 << self.order;

        if point.x >= size || point.y >= size {
            return Err(FaFsError::FractalError(
                "Point outside Hilbert grid".into(),
            ));
        }

        let mut x = point.x;
        let mut y = point.y;
        let mut distance = 0u64;
        let mut scale = size / 2;

        while scale > 0 {
            let rx = u32::from((x & scale) != 0);
            let ry = u32::from((y & scale) != 0);

            distance +=
                u64::from(scale) * u64::from(scale)
                * u64::from((3 * rx) ^ ry);

            if ry == 0 {
                if rx == 1 {
                    x = scale - 1 - x;
                    y = scale - 1 - y;
                }

                std::mem::swap(&mut x, &mut y);
            }

            scale /= 2;
        }

        Ok(distance)
    }

    pub fn decode(&self, distance: u64) -> FaFsResult<Point2D> {
        let size = 1u32 << self.order;
        let total = u64::from(size) * u64::from(size);

        if distance >= total {
            return Err(FaFsError::FractalError(
                "Hilbert distance outside valid range".into(),
            ));
        }

        let mut x = 0u32;
        let mut y = 0u32;
        let mut t = distance;
        let mut scale = 1u32;

        while scale < size {
            let rx = 1 & (t / 2) as u32;
            let ry = 1 & ((t ^ u64::from(rx)) as u32);

            if ry == 0 {
                if rx == 1 {
                    x = scale - 1 - x;
                    y = scale - 1 - y;
                }

                std::mem::swap(&mut x, &mut y);
            }

            x += scale * rx;
            y += scale * ry;

            t /= 4;
            scale *= 2;
        }

        Ok(Point2D { x, y })
    }
}

