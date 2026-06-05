// Fixture (#725): an exported runtime bootstrap that never installs the runtime
// probe global. The probe compatibility check must fail closed because the
// probe inspection surface is absent entirely.
'use strict';
(function () {
  let tick = 0;
  function advance() { tick += 1; return tick; }
  advance();
})();
