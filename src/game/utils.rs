pub fn len(v: [f32; 2]) -> f32
{
    f32::sqrt(f32::powi(v[0], 2) + f32::powi(v[1], 2))
}

pub fn sub(l: [f32; 2], r: [f32; 2]) -> [f32; 2]
{
    [l[0] - r[0], l[1] - r[1]]
}

pub fn div(v: [f32; 2], n: f32) -> [f32; 2]
{
    [v[0] / n, v[1] / n]
}

pub fn mul(v: [f32; 2], n: f32) -> [f32; 2]
{
    [v[0] * n, v[1] * n]
}

pub fn dot(l: [f32; 2], r: [f32; 2]) -> f32
{
    l[0] * r[0] + l[1] * r[1]
}

pub fn normalize(v: [f32; 2]) -> [f32; 2]
{
    div(v, len(v))
}

pub fn bounce_angle(v: [f32; 2], n: [f32; 2]) -> [f32; 2]
{
    let n = normalize(n);
    // 2(v . n)n - v
    sub(mul(mul(n, dot(n, v)), 2.0), v)
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

// distance to the center of the rectangle
pub fn dist_to_rect(p: [f32; 2], r: ggez::graphics::Rect) -> f32
{
    let c = [r.x + r.w / 2.0, r.y + r.h / 2.0];

    f32::sqrt(f32::powi(p[0] - c[0], 2) + f32::powi(p[1] - c[1], 2))
}

// if the point is inside the rect, returns the closest point that's outside the rect (with the size)
pub fn inside_rect(p: [f32; 2], s: f32, r: ggez::graphics::Rect) -> Option<[f32; 2]>
{
    if r.contains(p)
    {
        let dleft = f32::abs(p[0] - r.left());
        let dright = f32::abs(p[0] - r.right());
        let dtop = f32::abs(p[1] - r.top());
        let dbottom = f32::abs(p[1] - r.bottom());

        if dleft < f32::min(f32::min(dtop, dbottom), dright)
        {
            Some([r.left() - s, p[1]])
        }
        else if dright < f32::min(f32::min(dtop, dbottom), dleft)
        {
            Some([r.right() + s, p[1]])
        }
        else if dtop < f32::min(f32::min(dleft, dright), dbottom)
        {
            Some([p[0], r.top() - s])
        }
        else // dbottom < f32::min(f32::min(dleft, dright), dtop)
        {
            Some([p[0], r.bottom() + s])
        }
    }
    else
    {
        None
    }
}

pub fn rotate(v: [f32; 2], a: f32) -> [f32; 2]
{
    [
        f32::cos(a) * v[0] - f32::sin(a) * v[1],
        f32::sin(a) * v[0] + f32::cos(a) * v[1]
    ]
}

pub fn angle_between(v: [f32; 2], n: [f32; 2]) -> f32
{
    f32::acos(dot(v, n) / (len(v) * len(n)))
}
