<script>
	let clicked = 0;
	import { Form, Input, Select, Choice } from 'sveltejs-forms';
	import Spinner from 'svelte-spinner';
	import ChooseLocation from './ChooseLocation.svelte';

	import * as yup from 'yup';

	import { onMount } from 'svelte';

	let formValues;
	let position = {
		lat: 49.291676, 
		lng: 7.373426
	}


	function compareToB(x) {
		console.log("compareToB");
		console.log(x);
	}

	$: compareToB(position)

	async function handleSubmit({ detail: { values, setSubmitting, resetForm } }) {
		try {

			console.log("values", values)
			values.tree.lat = position.lat.toString();
			values.tree.lng = position.lng.toString();
			const options = {
				method: 'post',
				headers: {
					Accept: 'application/json',
					'Content-Type': 'application/json'
				},
				//make sure to serialize your JSON body
				body: JSON.stringify(values.tree)
			};
			const returnValue = await fetch(`http://localhost:5000/trees`, options);
			const response = await returnValue.json();
			// gifs = response.data;
			console.log('response', response);
			setSubmitting(true);
		} catch (error) {
			console.log('error', error);
		}

		formValues = values;
	}

	console.log('yp', yup);


	// https://wiki.openstreetmap.org/wiki/Tag:natural%3Dtree

	// An oak tree (unknown species):
	// leaf_type=broadleaved
	// genus=Quercus (this is the latin genus name)
	// genus:en=Oak

	const schema = yup.object().shape({
		tree: yup.object().shape({
			name: yup.string(),
			description: yup.string(),
			genus: yup.string().required(),
			lng: yup.number().required(),
			lat: yup.number().required(),
		})
	});

	// const genus = [
	// 	{ id: '1', title: 'Eiche (Oak)' },
	// 	{ id: '2', title: 'Fichte (Spruce)' },
	// 	{ id: '3', title: 'Birch (Birke)' },
	// 	{ id: '4', title: 'Buche (Beech)' }
	// ];
	const genus = [
		{ id: 'Eiche (Oak)', title: 'Eiche (Oak)' },
		{ id: 'Fichte (Spruce)', title: 'Fichte (Spruce)'},
		{ id: 'Birch (Birke)', title: 'Birch (Birke)'},
		{ id: 'Buche (Beech)', title: 'Buche (Beech)'}
	];
</script>

<Form
	{schema}
	validateOnChange={true}
	validateOnBlur={true}
	on:submit={handleSubmit}
	let:isSubmitting
>
	<Input name="tree.name" label="Name (Optional)" placeholder="e.g. Baum Nr 42" />

	<Input
		name="tree.description"
		label="Description  (Optional)"
		placeholder="noticeable features"
		multiline
	/>

	<label>Genus</label>
	<Choice name="tree.genus" options={genus} />

	<Input
		name="tree.lat"
		label="lat"
		bind:value={position.lat}
	/>
	<Input
		name="tree.lng"
		label="lng"
		bind:value={position.lng}
	/>

	<ChooseLocation bind:position={position} />

	
	<div class="buttons">
		<button type="submit" disabled={isSubmitting}>Submit</button>
		{#if isSubmitting}
			<Spinner />
		{/if}
	</div>
</Form>

<pre>{formValues}</pre>

<style>
	:global(.sveltejs-forms) {
		background-color: #f8f8f8;
		display: inline-block;
		padding: 1rem;
		border: 1px solid #ccc;
		border-radius: 5px;
	}

	:global(label) {
		font-size: 0.8rem;
		color: #888;
		margin-bottom: 0.2rem;
	}

	:global(.message) {
		font-size: 0.8rem;
		color: #888;
		margin: 0.2rem 0;
		color: #f56565;
	}

	:global(input[type='text']),
	:global(textarea),
	:global(select) {
		width: 100%;
		background-color: white;
		margin: 0;
	}

	:global(input[type='checkbox'] + label) {
		display: inline-block;
		margin-right: 2rem;
	}

	:global(.field) {
		margin-bottom: 1rem;
	}

	.buttons {
		margin-top: 0.8rem;
		display: flex;
		align-items: center;
	}

	button {
		border-radius: 5px;
		padding: 0.5rem 1rem;
		margin-right: 1rem;
		color: white;
	}

	button[type='reset'] {
		background-color: #f56565;
	}

	button[type='submit'] {
		background-color: #48bb78;
		width: 80px;
	}
</style>
