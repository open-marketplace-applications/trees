<script>
	import ImageList, {
		Item,
		ImageAspectContainer,
		Image,
		Supporting,
		Label
	} from '@smui/image-list';
</script>

<ImageList class="my-image-list-standard">
	{#each Array(15) as _unused, i}
		<Item>
			<ImageAspectContainer>
				<Image src="https://place-hold.it/190x190?text=square&fontsize=23" alt="Image {i + 1}" />
			</ImageAspectContainer>
			<Supporting>
				<Label>Image {i + 1}</Label>
			</Supporting>
		</Item>
	{/each}
</ImageList>

<style lang="scss">
	@use '@material/image-list/index' as image-list;

	.my-image-list-standard {
		@include image-list.standard-columns(5);
		max-width: 680px;
	}

	@media (max-width: 599px) {
		.my-image-list-standard {
			@include image-list.standard-columns(3);
		}
	}
</style>
