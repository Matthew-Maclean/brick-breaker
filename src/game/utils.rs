pub fn len(v: [f32; 2]) -> f32
{
    f32::sqrt(f32::powi(v[0], 2) + f32::powi(v[1], 2))
}

pub fn div(v: [f32; 2], n: f32) -> [f32; 2]
{
    [v[0] / n, v[1] / n]
}

pub fn normalize(v: [f32; 2]) -> [f32; 2]
{
    div(v, len(v))
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Axis
{
    X,
    Y,
}

pub fn bounce_axis(v: [f32; 2], axis: Axis) -> [f32; 2]
{
    match axis
    {
        Axis::X =>
        {
            [v[0], -v[1]]
        },
        Axis::Y =>
        {
            [-v[0], v[1]]
        },
    }
}

// o: old position, p: projected position, s: size, r: rect
pub fn intersect_rect(o: [f32; 2], p: [f32; 2], s: f32, r: ggez::graphics::Rect) -> Option<Axis>
{
    if r.contains(p)
    {
        // distance to either x face
        let xdist = f32::min(f32::abs(o[1] - r.top()), f32::abs(o[1] - r.bottom()));
        // distance to either y face
        let ydist = f32::min(f32::abs(o[0] - r.left()), f32::abs(o[0] - r.right()));

        if xdist < ydist
        {
            return Some(Axis::X)
        }
        else
        {
            return Some(Axis::Y)
        }
    }
    else
    {
        if p[0] > r.left() && p[0] < r.right()
        {
            if f32::abs(p[1] - r.top()) < s ||
                f32::abs(p[1] - r.bottom()) < s
            {
                return Some(Axis::X)
            }
        }
        if p[1] > r.top() && p[1] < r.bottom()
        {
            if f32::abs(p[0] - r.left()) < s ||
                f32::abs(p[0] - r.right()) < s
            {
                return Some(Axis::Y)
            }
        }

        let corner_intersect = |c: [f32; 2]|
        {
            f32::sqrt(f32::powi(p[0] - c[0], 2) + f32::powi(p[1] - c[1], 2)) < s
        };
        
        if corner_intersect([r.left(), r.top()])
        {
            if f32::abs(p[0] - r.left()) < f32::abs(p[1] - r.top())
            {
                return Some(Axis::X)
            }
            else
            {
                return Some(Axis::Y)
            }
        }
        if corner_intersect([r.left(), r.bottom()])
        {
            if f32::abs(p[0] - r.left()) < f32::abs(p[1] - r.bottom())
            {
                return Some(Axis::X)
            }
            else
            {
                return Some(Axis::Y)
            }
        }
        if corner_intersect([r.right(), r.top()])
        {
            if f32::abs(p[0] - r.right()) < f32::abs(p[1] - r.top())
            {
                return Some(Axis::X)
            }
            else
            {
                return Some(Axis::Y)
            }
        }
        if corner_intersect([r.right(), r.bottom()])
        {
            if f32::abs(p[0] - r.right()) < f32::abs(p[1] - r.bottom())
            {
                return Some(Axis::X)
            }
            else
            {
                return Some(Axis::Y)
            }
        }
    }

    None
}

pub fn dist_to_rect(p: [f32; 2], r: ggez::graphics::Rect) -> f32
{
    f32::min(
        f32::min(
            f32::abs(p[0] - r.left()),
            f32::abs(p[0] - r.right())
        ),
        f32::min(
            f32::abs(p[1] - r.top()),
            f32::abs(p[1] - r.bottom())
        ),
    )
}
