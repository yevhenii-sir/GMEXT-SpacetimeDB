function jwt_get_sub(token) {
	var dot1 = string_pos(".", token); 
	if (dot1 == 0) return undefined;

	var tail = string_copy(token, dot1 + 1, string_length(token) - dot1);

	var dot2_in_tail = string_pos(".", tail);
	if (dot2_in_tail == 0) return undefined;

	var payload_b64 = string_copy(token, dot1 + 1, dot2_in_tail - 1);
	payload_b64 = string_replace(payload_b64, "-", "+");
	payload_b64 = string_replace(payload_b64, "_", "/");
	var pad = string_length(payload_b64) mod 4;
	if (pad == 2) payload_b64 += "==";
	else if (pad == 3) payload_b64 += "=";

	var payload_json = base64_decode(payload_b64);
	if (payload_json == "") return undefined;

	var payload = json_parse(payload_json);
	if (is_undefined(payload) || is_undefined(payload[$ "hex_identity"])) return undefined;
	return "0x" + payload.hex_identity;
}
