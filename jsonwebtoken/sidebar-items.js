initSidebarItems({"enum":[["Algorithm","The algorithms supported for signing/verifying JWTs"]],"fn":[["decode","Decode and validate a JWT"],["decode_header","Decode a JWT without any signature verification/validations and return its Header."],["encode","Encode the header and claims given and sign the payload using the algorithm from the header and the key. If the algorithm given is RSA or EC, the key needs to be in the PEM format."],["get_current_timestamp","Gets the current timestamp in the format JWT expect"]],"mod":[["crypto","Lower level functions, if you want to do something other than JWTs"],["errors","All the errors that can be encountered while encoding/decoding JWTs"],["jwk",""]],"struct":[["DecodingKey","All the different kind of keys we can use to decode a JWT This key can be re-used so make sure you only initialize it once if you can for better performance"],["EncodingKey","A key to encode a JWT with. Can be a secret, a PEM-encoded key or a DER-encoded key. This key can be re-used so make sure you only initialize it once if you can for better performance"],["Header","A basic JWT header, the alg defaults to HS256 and typ is automatically set to `JWT`. All the other fields are optional."],["TokenData","The return type of a successful call to decode."],["Validation","Contains the various validations that are applied after decoding a JWT."]]});