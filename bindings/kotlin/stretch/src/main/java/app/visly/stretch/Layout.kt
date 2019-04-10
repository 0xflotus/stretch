package app.visly.stretch

data class Layout(
    val x: Float,
    val y: Float,
    val width: Float,
    val height: Float,
    val children: List<Layout>) {

    companion object {
        fun fromFloatArray(args: FloatArray, offset: Int): Pair<Int, Layout> {
            var offset = offset

            val x = args[offset++]
            val y = args[offset++]
            val width = args[offset++]
            val height = args[offset++]
            val childCount = args[offset++].toInt()
            val children = ArrayList<Layout>(childCount)

            for (i in 0..childCount) {
                val child = Layout.fromFloatArray(args, offset)
                offset = child.first
                children[i] = child.second
            }

            return Pair(offset, Layout(x, y, width, height, children))
        }
    }
}