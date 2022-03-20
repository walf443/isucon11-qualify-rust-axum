(function() {var implementors = {};
implementors["predicates"] = [{"text":"impl&lt;Item&gt; <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;Item&gt; for <a class=\"struct\" href=\"predicates/struct.BoxPredicate.html\" title=\"struct predicates::BoxPredicate\">BoxPredicate</a>&lt;Item&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Item: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,&nbsp;</span>","synthetic":false,"types":["predicates::boxed::BoxPredicate"]},{"text":"impl&lt;Item:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;Item&gt; for <a class=\"struct\" href=\"predicates/constant/struct.BooleanPredicate.html\" title=\"struct predicates::constant::BooleanPredicate\">BooleanPredicate</a>","synthetic":false,"types":["predicates::constant::BooleanPredicate"]},{"text":"impl&lt;F, T&gt; <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;T&gt; for <a class=\"struct\" href=\"predicates/function/struct.FnPredicate.html\" title=\"struct predicates::function::FnPredicate\">FnPredicate</a>&lt;F, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/ops/function/trait.Fn.html\" title=\"trait core::ops::function::Fn\">Fn</a>(<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.reference.html\">&amp;</a>T) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.bool.html\">bool</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,&nbsp;</span>","synthetic":false,"types":["predicates::function::FnPredicate"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;T&gt; for <a class=\"struct\" href=\"predicates/iter/struct.InPredicate.html\" title=\"struct predicates::iter::InPredicate\">InPredicate</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,&nbsp;</span>","synthetic":false,"types":["predicates::iter::InPredicate"]},{"text":"impl&lt;'a, T&gt; <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;T&gt; for <a class=\"struct\" href=\"predicates/iter/struct.InPredicate.html\" title=\"struct predicates::iter::InPredicate\">InPredicate</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.reference.html\">&amp;'a </a>T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,&nbsp;</span>","synthetic":false,"types":["predicates::iter::InPredicate"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;T&gt; for <a class=\"struct\" href=\"predicates/iter/struct.OrdInPredicate.html\" title=\"struct predicates::iter::OrdInPredicate\">OrdInPredicate</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,&nbsp;</span>","synthetic":false,"types":["predicates::iter::OrdInPredicate"]},{"text":"impl&lt;'a, T&gt; <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;T&gt; for <a class=\"struct\" href=\"predicates/iter/struct.OrdInPredicate.html\" title=\"struct predicates::iter::OrdInPredicate\">OrdInPredicate</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.reference.html\">&amp;'a </a>T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,&nbsp;</span>","synthetic":false,"types":["predicates::iter::OrdInPredicate"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;T&gt; for <a class=\"struct\" href=\"predicates/iter/struct.HashableInPredicate.html\" title=\"struct predicates::iter::HashableInPredicate\">HashableInPredicate</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,&nbsp;</span>","synthetic":false,"types":["predicates::iter::HashableInPredicate"]},{"text":"impl&lt;'a, T&gt; <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;T&gt; for <a class=\"struct\" href=\"predicates/iter/struct.HashableInPredicate.html\" title=\"struct predicates::iter::HashableInPredicate\">HashableInPredicate</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.reference.html\">&amp;'a </a>T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,&nbsp;</span>","synthetic":false,"types":["predicates::iter::HashableInPredicate"]},{"text":"impl&lt;M, Item&gt; <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;Item&gt; for <a class=\"struct\" href=\"predicates/name/struct.NamePredicate.html\" title=\"struct predicates::name::NamePredicate\">NamePredicate</a>&lt;M, Item&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;M: <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;Item&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Item: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,&nbsp;</span>","synthetic":false,"types":["predicates::name::NamePredicate"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;T&gt; for <a class=\"struct\" href=\"predicates/ord/struct.EqPredicate.html\" title=\"struct predicates::ord::EqPredicate\">EqPredicate</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>,&nbsp;</span>","synthetic":false,"types":["predicates::ord::EqPredicate"]},{"text":"impl&lt;'a, T&gt; <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;T&gt; for <a class=\"struct\" href=\"predicates/ord/struct.EqPredicate.html\" title=\"struct predicates::ord::EqPredicate\">EqPredicate</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.reference.html\">&amp;'a </a>T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,&nbsp;</span>","synthetic":false,"types":["predicates::ord::EqPredicate"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;T&gt; for <a class=\"struct\" href=\"predicates/ord/struct.OrdPredicate.html\" title=\"struct predicates::ord::OrdPredicate\">OrdPredicate</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>,&nbsp;</span>","synthetic":false,"types":["predicates::ord::OrdPredicate"]},{"text":"impl&lt;'a, T&gt; <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;T&gt; for <a class=\"struct\" href=\"predicates/ord/struct.OrdPredicate.html\" title=\"struct predicates::ord::OrdPredicate\">OrdPredicate</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.reference.html\">&amp;'a </a>T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,&nbsp;</span>","synthetic":false,"types":["predicates::ord::OrdPredicate"]},{"text":"impl&lt;M1, M2, Item&gt; <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;Item&gt; for <a class=\"struct\" href=\"predicates/boolean/struct.AndPredicate.html\" title=\"struct predicates::boolean::AndPredicate\">AndPredicate</a>&lt;M1, M2, Item&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;M1: <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;Item&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;M2: <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;Item&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Item: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,&nbsp;</span>","synthetic":false,"types":["predicates::boolean::AndPredicate"]},{"text":"impl&lt;M1, M2, Item&gt; <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;Item&gt; for <a class=\"struct\" href=\"predicates/boolean/struct.OrPredicate.html\" title=\"struct predicates::boolean::OrPredicate\">OrPredicate</a>&lt;M1, M2, Item&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;M1: <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;Item&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;M2: <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;Item&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Item: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,&nbsp;</span>","synthetic":false,"types":["predicates::boolean::OrPredicate"]},{"text":"impl&lt;M, Item&gt; <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;Item&gt; for <a class=\"struct\" href=\"predicates/boolean/struct.NotPredicate.html\" title=\"struct predicates::boolean::NotPredicate\">NotPredicate</a>&lt;M, Item&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;M: <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;Item&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Item: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,&nbsp;</span>","synthetic":false,"types":["predicates::boolean::NotPredicate"]},{"text":"impl <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.f64.html\">f64</a>&gt; for <a class=\"struct\" href=\"predicates/float/struct.IsClosePredicate.html\" title=\"struct predicates::float::IsClosePredicate\">IsClosePredicate</a>","synthetic":false,"types":["predicates::float::close::IsClosePredicate"]},{"text":"impl <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.59.0/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt; for <a class=\"struct\" href=\"predicates/path/struct.ExistencePredicate.html\" title=\"struct predicates::path::ExistencePredicate\">ExistencePredicate</a>","synthetic":false,"types":["predicates::path::existence::ExistencePredicate"]},{"text":"impl <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.59.0/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt; for <a class=\"struct\" href=\"predicates/path/struct.FileTypePredicate.html\" title=\"struct predicates::path::FileTypePredicate\">FileTypePredicate</a>","synthetic":false,"types":["predicates::path::ft::FileTypePredicate"]},{"text":"impl&lt;P&gt; <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.59.0/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt; for <a class=\"struct\" href=\"predicates/path/struct.FileContentPredicate.html\" title=\"struct predicates::path::FileContentPredicate\">FileContentPredicate</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.slice.html\">]</a>&gt;,&nbsp;</span>","synthetic":false,"types":["predicates::path::fc::FileContentPredicate"]},{"text":"impl <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.59.0/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt; for <a class=\"struct\" href=\"predicates/path/struct.BinaryFilePredicate.html\" title=\"struct predicates::path::BinaryFilePredicate\">BinaryFilePredicate</a>","synthetic":false,"types":["predicates::path::fs::BinaryFilePredicate"]},{"text":"impl <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"predicates/path/struct.BinaryFilePredicate.html\" title=\"struct predicates::path::BinaryFilePredicate\">BinaryFilePredicate</a>","synthetic":false,"types":["predicates::path::fs::BinaryFilePredicate"]},{"text":"impl <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.59.0/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt; for <a class=\"struct\" href=\"predicates/path/struct.StrFilePredicate.html\" title=\"struct predicates::path::StrFilePredicate\">StrFilePredicate</a>","synthetic":false,"types":["predicates::path::fs::StrFilePredicate"]},{"text":"impl <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"predicates/path/struct.StrFilePredicate.html\" title=\"struct predicates::path::StrFilePredicate\">StrFilePredicate</a>","synthetic":false,"types":["predicates::path::fs::StrFilePredicate"]},{"text":"impl <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"predicates/str/struct.IsEmptyPredicate.html\" title=\"struct predicates::str::IsEmptyPredicate\">IsEmptyPredicate</a>","synthetic":false,"types":["predicates::str::basics::IsEmptyPredicate"]},{"text":"impl <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"predicates/str/struct.StartsWithPredicate.html\" title=\"struct predicates::str::StartsWithPredicate\">StartsWithPredicate</a>","synthetic":false,"types":["predicates::str::basics::StartsWithPredicate"]},{"text":"impl <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"predicates/str/struct.EndsWithPredicate.html\" title=\"struct predicates::str::EndsWithPredicate\">EndsWithPredicate</a>","synthetic":false,"types":["predicates::str::basics::EndsWithPredicate"]},{"text":"impl <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"predicates/str/struct.ContainsPredicate.html\" title=\"struct predicates::str::ContainsPredicate\">ContainsPredicate</a>","synthetic":false,"types":["predicates::str::basics::ContainsPredicate"]},{"text":"impl <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"predicates/str/struct.MatchesPredicate.html\" title=\"struct predicates::str::MatchesPredicate\">MatchesPredicate</a>","synthetic":false,"types":["predicates::str::basics::MatchesPredicate"]},{"text":"impl&lt;P&gt; <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"predicates/str/struct.TrimPredicate.html\" title=\"struct predicates::str::TrimPredicate\">TrimPredicate</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.str.html\">str</a>&gt;,&nbsp;</span>","synthetic":false,"types":["predicates::str::adapters::TrimPredicate"]},{"text":"impl&lt;P&gt; <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.59.0/std/ffi/os_str/struct.OsStr.html\" title=\"struct std::ffi::os_str::OsStr\">OsStr</a>&gt; for <a class=\"struct\" href=\"predicates/str/struct.Utf8Predicate.html\" title=\"struct predicates::str::Utf8Predicate\">Utf8Predicate</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.str.html\">str</a>&gt;,&nbsp;</span>","synthetic":false,"types":["predicates::str::adapters::Utf8Predicate"]},{"text":"impl&lt;P&gt; <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"predicates/str/struct.Utf8Predicate.html\" title=\"struct predicates::str::Utf8Predicate\">Utf8Predicate</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.str.html\">str</a>&gt;,&nbsp;</span>","synthetic":false,"types":["predicates::str::adapters::Utf8Predicate"]},{"text":"impl <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"predicates/str/struct.DifferencePredicate.html\" title=\"struct predicates::str::DifferencePredicate\">DifferencePredicate</a>","synthetic":false,"types":["predicates::str::difference::DifferencePredicate"]},{"text":"impl&lt;P&gt; <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"predicates/str/struct.NormalizedPredicate.html\" title=\"struct predicates::str::NormalizedPredicate\">NormalizedPredicate</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.str.html\">str</a>&gt;,&nbsp;</span>","synthetic":false,"types":["predicates::str::normalize::NormalizedPredicate"]},{"text":"impl <a class=\"trait\" href=\"predicates/trait.Predicate.html\" title=\"trait predicates::Predicate\">Predicate</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.59.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"predicates/str/struct.RegexPredicate.html\" title=\"struct predicates::str::RegexPredicate\">RegexPredicate</a>","synthetic":false,"types":["predicates::str::regex::RegexPredicate"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()