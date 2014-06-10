macro_rules! byte_var(
  (
    $name:ident = $literal:expr
  ) => (
    static $name: &'static [u8] = bytes!( $literal );
  );
)

macro_rules! data( ( $ex:expr ) => ( {
      byte_var!( input = $ex )
      Data( input )
    } ) )

