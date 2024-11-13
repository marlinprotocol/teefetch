// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {Teefetch} from "../../src/Teefetch.sol";
import "../../lib/risc0-ethereum/contracts/src/IRiscZeroVerifier.sol";

contract DeployScript is Script {
    IRiscZeroVerifier private constant VERIFIER =
        IRiscZeroVerifier(0x0b144E07A0826182B6b59788c34b32Bfa86Fb711);
    bytes32 private constant IMAGE_ID =
        0xdce6b83ae4bdcf22edbd23b86762ce08e98b1767eab21cdd3bb9d4d1c4d3e2b8;
    bytes private constant PCRS =
        hex"bf2858ff0044a23022ca495f52c82254aca0c7121271363216587182571550872f34e0ddf76f868a185751e7961ee2833c9d303f89856ec3410913381c328350c32d14d2f86a2b4a7787998bd6d76d8f60fc88fea094bf5a02b2c2df1b7ad832d2f59b40c919d71d9033bbc6e9f62b62bc874e65d04414a610681a53a5641af44d177cfadeae804f820861b5680c1926";
    bytes private constant ROOT_KEY =
        hex"04fc0254eba608c1f36870e29ada90be46383292736e894bfff672d989444b5051e534a4b1f6dbe3c0bc581a32b7b176070ede12d69a3fea211b66e752cf7dd1dd095f6f1370f4170843d9dc100121e4cf63012809664487c9796284304dc53ff4";

    function run() external returns (Teefetch) {
        vm.startBroadcast();

        Teefetch teefetch = new Teefetch(VERIFIER, IMAGE_ID, PCRS, ROOT_KEY);

        vm.stopBroadcast();

        console.log("Deployed to:", address(teefetch));

        return teefetch;
    }
}
