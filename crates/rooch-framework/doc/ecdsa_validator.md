
<a name="0x3_ecdsa_validator"></a>

# Module `0x3::ecdsa_validator`

This module implements the ecdsa validator scheme.


-  [Struct `EcdsaValidator`](#0x3_ecdsa_validator_EcdsaValidator)
-  [Constants](#@Constants_0)
-  [Function `scheme`](#0x3_ecdsa_validator_scheme)
-  [Function `validate`](#0x3_ecdsa_validator_validate)


<pre><code><b>use</b> <a href="">0x2::storage_context</a>;
<b>use</b> <a href="auth_validator.md#0x3_auth_validator">0x3::auth_validator</a>;
<b>use</b> <a href="ecdsa_k1.md#0x3_ecdsa_k1">0x3::ecdsa_k1</a>;
</code></pre>



<a name="0x3_ecdsa_validator_EcdsaValidator"></a>

## Struct `EcdsaValidator`



<pre><code><b>struct</b> <a href="ecdsa_validator.md#0x3_ecdsa_validator_EcdsaValidator">EcdsaValidator</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dummy_field: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x3_ecdsa_validator_SCHEME_ECDSA"></a>



<pre><code><b>const</b> <a href="ecdsa_validator.md#0x3_ecdsa_validator_SCHEME_ECDSA">SCHEME_ECDSA</a>: u64 = 2;
</code></pre>



<a name="0x3_ecdsa_validator_scheme"></a>

## Function `scheme`



<pre><code><b>public</b> <b>fun</b> <a href="ecdsa_validator.md#0x3_ecdsa_validator_scheme">scheme</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="ecdsa_validator.md#0x3_ecdsa_validator_scheme">scheme</a>(): u64 {
   <a href="ecdsa_validator.md#0x3_ecdsa_validator_SCHEME_ECDSA">SCHEME_ECDSA</a>
}
</code></pre>



</details>

<a name="0x3_ecdsa_validator_validate"></a>

## Function `validate`



<pre><code><b>public</b> <b>fun</b> <a href="ecdsa_validator.md#0x3_ecdsa_validator_validate">validate</a>(ctx: &<a href="_StorageContext">storage_context::StorageContext</a>, payload: <a href="">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="ecdsa_validator.md#0x3_ecdsa_validator_validate">validate</a>(ctx: &StorageContext, payload: <a href="">vector</a>&lt;u8&gt;){
   //FIXME check the <b>address</b> and <b>public</b> key relationship
   <b>assert</b>!(
   <a href="ecdsa_k1.md#0x3_ecdsa_k1_verify">ecdsa_k1::verify</a>(
         &payload,
         &<a href="_tx_hash">storage_context::tx_hash</a>(ctx),
         0 // KECCAK256:0, SHA256:1, TODO: The <a href="../doc/hash.md#0x1_hash">hash</a> type may need <b>to</b> be passed through the authenticator
   ),
   <a href="auth_validator.md#0x3_auth_validator_error_invalid_authenticator">auth_validator::error_invalid_authenticator</a>());
}
</code></pre>



</details>
