const GOLDEN_RATIO: u32 = 0x9e3779b9;
const LOG_SIZE: u32 = 8;
const SIZE: usize = 1 << LOG_SIZE;
const MASK: u32 = (SIZE as u32 - 1) << 2;

pub struct IsaacRandom {
	a: u32,
	b: u32,
	c: u32,
	count: u32,
	pub mem: [u32; SIZE],
	rsl: [u32; SIZE],
}

impl IsaacRandom {
	#[inline]
	pub fn new(seed: &[u32]) -> Self {
		let mem = [0; SIZE];
		let mut rsl = [0; SIZE];
		
		rsl[..seed.len()].clone_from_slice(&seed[..]);

		let mut isaac = Self { a: 0, b: 0, c: 0, count: 0, mem, rsl };
		isaac.init();
		
		isaac
	}
	
	fn init(&mut self) {
		let mut h = GOLDEN_RATIO;
		let mut g = h;
		let mut f = g;
		let mut e = f;
		let mut d = e;
		let mut c = d;
		let mut b = c;
		let mut a = b;

		let mut i = 0;
		while i < 4 {
			a ^= b << 11;
			d = d.wrapping_add(a);
			b = b.wrapping_add(c);
			b ^= c >> 2;
			e = e.wrapping_add(b);
			c = c.wrapping_add(d);
			c ^= d << 8;
			f = f.wrapping_add(c);
			d = d.wrapping_add(e);
			d ^= e >> 16;
			g = g.wrapping_add(d);
			e = e.wrapping_add(f);
			e ^= f << 10;
			h = h.wrapping_add(e);
			f = f.wrapping_add(g);
			f ^= g >> 4;
			a = a.wrapping_add(f);
			g = g.wrapping_add(h);
			g ^= h << 8;
			b = b.wrapping_add(g);
			h = h.wrapping_add(a);
			h ^= a >> 9;
			c = c.wrapping_add(h);
			a = a.wrapping_add(b);
			i += 1;
		}

		i = 0;
		while i < SIZE {
			a = a.wrapping_add(self.rsl[i]);
			b = b.wrapping_add(self.rsl[i + 1]);
			c = c.wrapping_add(self.rsl[i + 2]);
			d = d.wrapping_add(self.rsl[i + 3]);
			e = e.wrapping_add(self.rsl[i + 4]);
			f = f.wrapping_add(self.rsl[i + 5]);
			g = g.wrapping_add(self.rsl[i + 6]);
			h = h.wrapping_add(self.rsl[i + 7]);

			a ^= b << 11;
			d = d.wrapping_add(a);
			b = b.wrapping_add(c);
			b ^= c >> 2;
			e = e.wrapping_add(b);
			c = c.wrapping_add(d);
			c ^= d << 8;
			f = f.wrapping_add(c);
			d = d.wrapping_add(e);
			d ^= e >> 16;
			g = g.wrapping_add(d);
			e = e.wrapping_add(f);
			e ^= f << 10;
			h = h.wrapping_add(e);
			f = f.wrapping_add(g);
			f ^= g >> 4;
			a = a.wrapping_add(f);
			g = g.wrapping_add(h);
			g ^= h << 8;
			b = b.wrapping_add(g);
			h = h.wrapping_add(a);
			h ^= a >> 9;
			c = c.wrapping_add(h);
			a = a.wrapping_add(b);
			self.mem[i] = a;
			self.mem[i + 1] = b;
			self.mem[i + 2] = c;
			self.mem[i + 3] = d;
			self.mem[i + 4] = e;
			self.mem[i + 5] = f;
			self.mem[i + 6] = g;
			self.mem[i + 7] = h;
			i += 8;
		}

		i = 0;
		while i < SIZE {
			a = a.wrapping_add(self.mem[i]);
			b = b.wrapping_add(self.mem[i + 1]);
			c = c.wrapping_add(self.mem[i + 2]);
			d = d.wrapping_add(self.mem[i + 3]);
			e = e.wrapping_add(self.mem[i + 4]);
			f = f.wrapping_add(self.mem[i + 5]);
			g = g.wrapping_add(self.mem[i + 6]);
			h = h.wrapping_add(self.mem[i + 7]);
			a ^= b << 11;
			d = d.wrapping_add(a);
			b = b.wrapping_add(c);
			b ^= c >> 2;
			e = e.wrapping_add(b);
			c = c.wrapping_add(d);
			c ^= d << 8;
			f = f.wrapping_add(c);
			d = d.wrapping_add(e);
			d ^= e >> 16;
			g = g.wrapping_add(d);
			e = e.wrapping_add(f);
			e ^= f << 10;
			h = h.wrapping_add(e);
			f = f.wrapping_add(g);
			f ^= g >> 4;
			a = a.wrapping_add(f);
			g = g.wrapping_add(h);
			g ^= h << 8;
			b = b.wrapping_add(g);
			h = h.wrapping_add(a);
			h ^= a >> 9;
			c = c.wrapping_add(h);
			a = a.wrapping_add(b);
			self.mem[i] = a;
			self.mem[i + 1] = b;
			self.mem[i + 2] = c;
			self.mem[i + 3] = d;
			self.mem[i + 4] = e;
			self.mem[i + 5] = f;
			self.mem[i + 6] = g;
			self.mem[i + 7] = h;
			i += 8;
		}

		self.isaac();
		self.count = SIZE as u32;
	}

	fn isaac(&mut self) {
		let mut i = 0;
		let mut j = SIZE / 2;
		let mut x;
		let mut y;

		self.c += 1;
		self.b += self.c;
		while i < SIZE / 2 {
			x = self.mem[i];
			self.a = self.a ^ (self.a << 13);
			self.a = self.a.wrapping_add(self.mem[j]);
			j += 1;
			y = self.mem[((x & MASK) >> 2) as usize].wrapping_add(self.a).wrapping_add(self.b);
			self.mem[i] = y;
			self.b = self.mem[(((y >> LOG_SIZE) & MASK) >> 2) as usize].wrapping_add(x);
			self.rsl[i] = self.b;
			i += 1;

			x = self.mem[i];
			self.a = self.a ^ self.a >> 6;
			self.a = self.a.wrapping_add(self.mem[j]);
			j += 1;
			y = self.mem[((x & MASK) >> 2) as usize].wrapping_add(self.a).wrapping_add(self.b);
			self.mem[i] = y;
			self.b = self.mem[(((y >> LOG_SIZE) & MASK) >> 2) as usize].wrapping_add(x);
			self.rsl[i] = self.b;
			i += 1;

			x = self.mem[i];
			self.a = self.a ^ (self.a << 2);
			self.a = self.a.wrapping_add(self.mem[j]);
			j += 1;
			y = self.mem[((x & MASK) >> 2) as usize].wrapping_add(self.a).wrapping_add(self.b);
			self.mem[i] = y;
			self.b = self.mem[(((y >> LOG_SIZE) & MASK) >> 2) as usize].wrapping_add(x);
			self.rsl[i] = self.b;
			i += 1;

			x = self.mem[i];
			self.a = self.a ^ self.a >> 16;
			self.a = self.a.wrapping_add(self.mem[j]);
			j += 1;
			y = self.mem[((x & MASK) >> 2) as usize].wrapping_add(self.a).wrapping_add(self.b);
			self.mem[i] = y;
			self.b = self.mem[(((y >> LOG_SIZE) & MASK) >> 2) as usize].wrapping_add(x);
			self.rsl[i] = self.b;
			i += 1;
		}

		j = 0;
		while j < SIZE / 2 {
			x = self.mem[i];
			self.a = self.a ^ (self.a << 13);
			self.a = self.a.wrapping_add(self.mem[j]);
			j += 1;
			y = self.mem[((x & MASK) >> 2) as usize].wrapping_add(self.a).wrapping_add(self.b);
			self.mem[i] = y;
			self.b = self.mem[(((y >> LOG_SIZE) & MASK) >> 2) as usize].wrapping_add(x);
			self.rsl[i] = self.b;
			i += 1;

			x = self.mem[i];
			self.a = self.a ^ self.a >> 6;
			self.a = self.a.wrapping_add(self.mem[j]);
			j += 1;
			y = self.mem[((x & MASK) >> 2) as usize].wrapping_add(self.a).wrapping_add(self.b);
			self.mem[i] = y;
			self.b = self.mem[(((y >> LOG_SIZE) & MASK) >> 2) as usize].wrapping_add(x);
			self.rsl[i] = self.b;
			i += 1;

			x = self.mem[i];
			self.a = self.a ^ (self.a << 2);
			self.a = self.a.wrapping_add(self.mem[j]);
			j += 1;
			y = self.mem[((x & MASK) >> 2) as usize].wrapping_add(self.a).wrapping_add(self.b);
			self.mem[i] = y;
			self.b = self.mem[(((y >> LOG_SIZE) & MASK) >> 2) as usize].wrapping_add(x);
			self.rsl[i] = self.b;
			i += 1;

			x = self.mem[i];
			self.a = self.a ^ self.a >> 16;
			self.a = self.a.wrapping_add(self.mem[j]);
			j += 1;
			y = self.mem[((x & MASK) >> 2) as usize].wrapping_add(self.a).wrapping_add(self.b);
			self.mem[i] = y;
			self.b = self.mem[(((y >> LOG_SIZE) & MASK) >> 2) as usize].wrapping_add(x);
			self.rsl[i] = self.b;
			i += 1;
		}
	}
}

impl Iterator for IsaacRandom {
	type Item = u32;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		if 0 == self.count {
			self.isaac();
			self.count = SIZE as u32;
		}
		self.count -= 1;

		Some(self.rsl[self.count as usize])
	}
}