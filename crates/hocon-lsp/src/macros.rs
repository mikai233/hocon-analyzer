#[macro_export(local_inner_macros)]
macro_rules! node_scope {
    ($builder:expr, $kind:expr, $($content:tt)*) => {
        {
            $builder.start_node($kind.into());
            let res = $($content)*;
            $builder.finish_node();
            res
        }
    };
}
