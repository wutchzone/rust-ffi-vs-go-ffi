#include "test.h"
#include <libavformat/avformat.h>
#include <libavutil/timestamp.h>

int rust_transmuxer(callback read, callback write) {
  unsigned char *buffer = av_malloc(8192);
  unsigned char *o_buf = av_malloc(2 * 8192);
  // used for input file
  AVIOContext *io_context =
      avio_alloc_context(buffer, 8192, 0, NULL, read, NULL, NULL);
  // used for ouput file
  AVIOContext *ioo_context =
      avio_alloc_context(o_buf, 2 * 8192, 1, NULL, NULL, write, NULL);

  int ret, i;
  AVFormatContext *f_context = avformat_alloc_context(),
                  *o_context = avformat_alloc_context();
  f_context->pb = io_context;

  if ((ret = avformat_open_input(&f_context, "show me your boobs", NULL,
                                 NULL)) < 0) {
    fprintf(stderr, "Could not open input file\n");
    return 1;
  }
  if ((ret = avformat_find_stream_info(f_context, NULL)) < 0) {
    fprintf(stderr, "Failed to retrieve input stream information");
    return 1;
  }

  // prepare output
  if (avformat_alloc_output_context2(&o_context, NULL, "mpegts", NULL) < 0) {
    fprintf(stderr, "error during inicialization of ouput");
    return 1;
  }
  o_context->pb = ioo_context;

  // copy codecs info
  int *streams_list =
      av_mallocz_array(f_context->nb_streams, sizeof(*streams_list));
  int stream_index = 0;
  for (i = 0; (unsigned int)i < f_context->nb_streams; i++) {
    AVStream *out_stream;
    AVStream *in_stream = f_context->streams[i];
    AVCodecParameters *in_codecpar = in_stream->codecpar;
    if (in_codecpar->codec_type != AVMEDIA_TYPE_AUDIO &&
        in_codecpar->codec_type != AVMEDIA_TYPE_VIDEO &&
        in_codecpar->codec_type != AVMEDIA_TYPE_SUBTITLE) {
      streams_list[i] = -1;
      continue;
    }
    streams_list[i] = stream_index++;
    out_stream = avformat_new_stream(o_context, NULL);
    if (!out_stream) {
      fprintf(stderr, "Failed allocating output stream\n");
      return 1;
    }

    // copy codec info to new context
    if (avcodec_parameters_copy(out_stream->codecpar, in_codecpar) < 0) {
      fprintf(stderr, "Failed to copy codec parameters\n");
      return 1;
    }
  }

  // write header
  if (avformat_write_header(o_context, NULL) < 0) {
    fprintf(stderr, "failed to write a header");
    return 1;
  }

  // transmuxing
  AVPacket packet;
  while (1) {
    AVStream *in_stream, *out_stream;
    ret = av_read_frame(f_context, &packet);
    if (ret < 0)
      break;
    in_stream = f_context->streams[packet.stream_index];
    if ((unsigned int)packet.stream_index >= f_context->nb_streams ||
        streams_list[packet.stream_index] < 0) {
      av_packet_unref(&packet);
      continue;
    }
    packet.stream_index = streams_list[packet.stream_index];
    out_stream = o_context->streams[packet.stream_index];
    /* copy packet */
    packet.pts = av_rescale_q_rnd(packet.pts, in_stream->time_base,
                                  out_stream->time_base,
                                  AV_ROUND_NEAR_INF | AV_ROUND_PASS_MINMAX);
    packet.dts = av_rescale_q_rnd(packet.dts, in_stream->time_base,
                                  out_stream->time_base,
                                  AV_ROUND_NEAR_INF | AV_ROUND_PASS_MINMAX);
    packet.duration = av_rescale_q(packet.duration, in_stream->time_base,
                                   out_stream->time_base);
    packet.pos = -1; // unknown

    ret = av_interleaved_write_frame(o_context, &packet);
    if (ret < 0) {
      fprintf(stderr, "Error muxing packet\n");
      break;
    }
    av_packet_unref(&packet);
  }

  // cleanup
  av_freep(&streams_list);
  // printf("everything done number of streams %d\n", f_context->nb_streams);
  avformat_free_context(f_context);
  avformat_free_context(o_context);
  av_free(io_context->buffer);

  return 0;
}
