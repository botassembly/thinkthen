import 'dart:ffi';
import 'abi.dart';

final class NativeApi {
  final DynamicLibrary lib;
  NativeApi(String path) : lib = DynamicLibrary.open(path);
  late final Pointer<Void> Function() thinkthen_engine_new =
      lib.lookupFunction<Pointer<Void> Function(), Pointer<Void> Function()>(
          "thinkthen_engine_new");
  late final Pointer<Void> Function(Pointer<Void>) thinkthen_engine_new_with =
      lib.lookupFunction<Pointer<Void> Function(Pointer<Void>),
          Pointer<Void> Function(Pointer<Void>)>("thinkthen_engine_new_with");
  late final void Function(Pointer<Void>) thinkthen_engine_free =
      lib.lookupFunction<Void Function(Pointer<Void>),
          void Function(Pointer<Void>)>("thinkthen_engine_free");
  late final Pointer<Void> Function() thinkthen_cancel_token_new =
      lib.lookupFunction<Pointer<Void> Function(), Pointer<Void> Function()>(
          "thinkthen_cancel_token_new");
  late final void Function(Pointer<Void>) thinkthen_cancel = lib.lookupFunction<
      Void Function(Pointer<Void>),
      void Function(Pointer<Void>)>("thinkthen_cancel");
  late final void Function(Pointer<Void>) thinkthen_cancel_token_free =
      lib.lookupFunction<Void Function(Pointer<Void>),
          void Function(Pointer<Void>)>("thinkthen_cancel_token_free");
  late final int Function(Pointer<Void>, int, Pointer<CSourceRecognitionView>)
      thinkthen_result_source_recognition = lib
          .lookupFunction<
                  Int32 Function(
                      Pointer<Void>, Size, Pointer<CSourceRecognitionView>),
                  int Function(
                      Pointer<Void>, int, Pointer<CSourceRecognitionView>)>(
              "thinkthen_result_source_recognition");
  late final int Function(Pointer<Void>, int, Pointer<CSourceRelationsView>)
      thinkthen_result_source_relations = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Size, Pointer<CSourceRelationsView>),
          int Function(
              Pointer<Void>,
              int,
              Pointer<
                  CSourceRelationsView>)>("thinkthen_result_source_relations");
  late final int Function(Pointer<Void>, int, Pointer<CDetailsView>)
      thinkthen_result_details = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Size, Pointer<CDetailsView>),
          int Function(Pointer<Void>, int,
              Pointer<CDetailsView>)>("thinkthen_result_details");
  late final int Function(Pointer<Void>, int, Pointer<CDetailsView>)
      thinkthen_result_observation_details = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Size, Pointer<CDetailsView>),
          int Function(Pointer<Void>, int,
              Pointer<CDetailsView>)>("thinkthen_result_observation_details");
  late final int Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
          Pointer<CControlsView>, Pointer<Pointer<Void>>)
      thinkthen_decide_batch_start = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
              Pointer<CControlsView>, Pointer<Pointer<Void>>),
          int Function(
              Pointer<Void>,
              Pointer<Void>,
              Pointer<Void>,
              Pointer<CControlsView>,
              Pointer<Pointer<Void>>)>("thinkthen_decide_batch_start");
  late final int Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
          Pointer<CControlsView>, Pointer<Pointer<Void>>)
      thinkthen_choose_batch_start = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
              Pointer<CControlsView>, Pointer<Pointer<Void>>),
          int Function(
              Pointer<Void>,
              Pointer<Void>,
              Pointer<Void>,
              Pointer<CControlsView>,
              Pointer<Pointer<Void>>)>("thinkthen_choose_batch_start");
  late final int Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
          Pointer<CControlsView>, Pointer<Pointer<Void>>)
      thinkthen_tag_batch_start = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
              Pointer<CControlsView>, Pointer<Pointer<Void>>),
          int Function(
              Pointer<Void>,
              Pointer<Void>,
              Pointer<Void>,
              Pointer<CControlsView>,
              Pointer<Pointer<Void>>)>("thinkthen_tag_batch_start");
  late final int Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
          Pointer<CControlsView>, Pointer<Pointer<Void>>)
      thinkthen_score_batch_start = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
              Pointer<CControlsView>, Pointer<Pointer<Void>>),
          int Function(
              Pointer<Void>,
              Pointer<Void>,
              Pointer<Void>,
              Pointer<CControlsView>,
              Pointer<Pointer<Void>>)>("thinkthen_score_batch_start");
  late final int Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
          Pointer<CControlsView>, Pointer<Pointer<Void>>)
      thinkthen_filter_batch_start = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
              Pointer<CControlsView>, Pointer<Pointer<Void>>),
          int Function(
              Pointer<Void>,
              Pointer<Void>,
              Pointer<Void>,
              Pointer<CControlsView>,
              Pointer<Pointer<Void>>)>("thinkthen_filter_batch_start");
  late final int Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
          Pointer<CControlsView>, Pointer<Pointer<Void>>)
      thinkthen_annotate_batch_start = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
              Pointer<CControlsView>, Pointer<Pointer<Void>>),
          int Function(
              Pointer<Void>,
              Pointer<Void>,
              Pointer<Void>,
              Pointer<CControlsView>,
              Pointer<Pointer<Void>>)>("thinkthen_annotate_batch_start");
  late final int Function(Pointer<Void>, Pointer<Pointer<Void>>)
      thinkthen_batch_next = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Pointer<Void>>),
          int Function(
              Pointer<Void>, Pointer<Pointer<Void>>)>("thinkthen_batch_next");
  late final int Function(Pointer<Void>, Pointer<Pointer<Void>>)
      thinkthen_batch_facts = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Pointer<Void>>),
          int Function(
              Pointer<Void>, Pointer<Pointer<Void>>)>("thinkthen_batch_facts");
  late final void Function(Pointer<Void>) thinkthen_batch_free =
      lib.lookupFunction<Void Function(Pointer<Void>),
          void Function(Pointer<Void>)>("thinkthen_batch_free");
  late final int Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
          Pointer<CControlsView>, Pointer<Pointer<Void>>)
      thinkthen_decide_complete = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
              Pointer<CControlsView>, Pointer<Pointer<Void>>),
          int Function(
              Pointer<Void>,
              Pointer<Void>,
              Pointer<Void>,
              Pointer<CControlsView>,
              Pointer<Pointer<Void>>)>("thinkthen_decide_complete");
  late final int Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
          Pointer<CControlsView>, Pointer<Pointer<Void>>)
      thinkthen_choose_complete = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
              Pointer<CControlsView>, Pointer<Pointer<Void>>),
          int Function(
              Pointer<Void>,
              Pointer<Void>,
              Pointer<Void>,
              Pointer<CControlsView>,
              Pointer<Pointer<Void>>)>("thinkthen_choose_complete");
  late final int Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
          Pointer<CControlsView>, Pointer<Pointer<Void>>)
      thinkthen_tag_complete = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
              Pointer<CControlsView>, Pointer<Pointer<Void>>),
          int Function(
              Pointer<Void>,
              Pointer<Void>,
              Pointer<Void>,
              Pointer<CControlsView>,
              Pointer<Pointer<Void>>)>("thinkthen_tag_complete");
  late final int Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
          Pointer<CControlsView>, Pointer<Pointer<Void>>)
      thinkthen_score_complete = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
              Pointer<CControlsView>, Pointer<Pointer<Void>>),
          int Function(
              Pointer<Void>,
              Pointer<Void>,
              Pointer<Void>,
              Pointer<CControlsView>,
              Pointer<Pointer<Void>>)>("thinkthen_score_complete");
  late final int Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
          Pointer<CControlsView>, Pointer<Pointer<Void>>)
      thinkthen_filter_complete = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
              Pointer<CControlsView>, Pointer<Pointer<Void>>),
          int Function(
              Pointer<Void>,
              Pointer<Void>,
              Pointer<Void>,
              Pointer<CControlsView>,
              Pointer<Pointer<Void>>)>("thinkthen_filter_complete");
  late final int Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
          Pointer<CControlsView>, Pointer<Pointer<Void>>)
      thinkthen_rank_complete = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
              Pointer<CControlsView>, Pointer<Pointer<Void>>),
          int Function(
              Pointer<Void>,
              Pointer<Void>,
              Pointer<Void>,
              Pointer<CControlsView>,
              Pointer<Pointer<Void>>)>("thinkthen_rank_complete");
  late final int Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
          Pointer<CControlsView>, Pointer<Pointer<Void>>)
      thinkthen_find_complete = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
              Pointer<CControlsView>, Pointer<Pointer<Void>>),
          int Function(
              Pointer<Void>,
              Pointer<Void>,
              Pointer<Void>,
              Pointer<CControlsView>,
              Pointer<Pointer<Void>>)>("thinkthen_find_complete");
  late final int Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
          Pointer<CControlsView>, Pointer<Pointer<Void>>)
      thinkthen_annotate_complete = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
              Pointer<CControlsView>, Pointer<Pointer<Void>>),
          int Function(
              Pointer<Void>,
              Pointer<Void>,
              Pointer<Void>,
              Pointer<CControlsView>,
              Pointer<Pointer<Void>>)>("thinkthen_annotate_complete");
  late final int Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
          Pointer<CControlsView>, Pointer<Pointer<Void>>)
      thinkthen_recognize_complete = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
              Pointer<CControlsView>, Pointer<Pointer<Void>>),
          int Function(
              Pointer<Void>,
              Pointer<Void>,
              Pointer<Void>,
              Pointer<CControlsView>,
              Pointer<Pointer<Void>>)>("thinkthen_recognize_complete");
  late final int Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
          Pointer<CControlsView>, Pointer<Pointer<Void>>)
      thinkthen_relate_complete = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
              Pointer<CControlsView>, Pointer<Pointer<Void>>),
          int Function(
              Pointer<Void>,
              Pointer<Void>,
              Pointer<Void>,
              Pointer<CControlsView>,
              Pointer<Pointer<Void>>)>("thinkthen_relate_complete");
  late final void Function(Pointer<Void>) thinkthen_result_free =
      lib.lookupFunction<Void Function(Pointer<Void>),
          void Function(Pointer<Void>)>("thinkthen_result_free");
  late final int Function(Pointer<Void>, Pointer<CSummaryView>)
      thinkthen_result_summary = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<CSummaryView>),
          int Function(Pointer<Void>,
              Pointer<CSummaryView>)>("thinkthen_result_summary");
  late final int Function(Pointer<Void>, int, Pointer<CObservationView>)
      thinkthen_result_observation = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Size, Pointer<CObservationView>),
          int Function(Pointer<Void>, int,
              Pointer<CObservationView>)>("thinkthen_result_observation");
  late final int Function(Pointer<Void>, int, Pointer<CDecideView>)
      thinkthen_result_decide = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Size, Pointer<CDecideView>),
          int Function(Pointer<Void>, int,
              Pointer<CDecideView>)>("thinkthen_result_decide");
  late final int Function(Pointer<Void>, int, Pointer<CChooseView>)
      thinkthen_result_choose = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Size, Pointer<CChooseView>),
          int Function(Pointer<Void>, int,
              Pointer<CChooseView>)>("thinkthen_result_choose");
  late final int Function(Pointer<Void>, int, Pointer<CTagView>)
      thinkthen_result_tag = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Size, Pointer<CTagView>),
          int Function(
              Pointer<Void>, int, Pointer<CTagView>)>("thinkthen_result_tag");
  late final int Function(Pointer<Void>, int, Pointer<CScoreView>)
      thinkthen_result_score = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Size, Pointer<CScoreView>),
          int Function(Pointer<Void>, int,
              Pointer<CScoreView>)>("thinkthen_result_score");
  late final int Function(Pointer<Void>, int, Pointer<CFilterView>)
      thinkthen_result_filter = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Size, Pointer<CFilterView>),
          int Function(Pointer<Void>, int,
              Pointer<CFilterView>)>("thinkthen_result_filter");
  late final int Function(Pointer<Void>, int, Pointer<CRankView>)
      thinkthen_result_rank = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Size, Pointer<CRankView>),
          int Function(
              Pointer<Void>, int, Pointer<CRankView>)>("thinkthen_result_rank");
  late final int Function(Pointer<Void>, int, Pointer<Size>)
      thinkthen_result_rank_member_count = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Size, Pointer<Size>),
          int Function(Pointer<Void>, int,
              Pointer<Size>)>("thinkthen_result_rank_member_count");
  late final int Function(Pointer<Void>, int, int, Pointer<CDetailsView>)
      thinkthen_result_rank_member_details = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Size, Size, Pointer<CDetailsView>),
          int Function(Pointer<Void>, int, int,
              Pointer<CDetailsView>)>("thinkthen_result_rank_member_details");
  late final int Function(Pointer<Void>, int, int, Pointer<CRankView>)
      thinkthen_result_rank_member = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Size, Size, Pointer<CRankView>),
          int Function(Pointer<Void>, int, int,
              Pointer<CRankView>)>("thinkthen_result_rank_member");
  late final int Function(Pointer<Void>, int, Pointer<CFindView>)
      thinkthen_result_find = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Size, Pointer<CFindView>),
          int Function(
              Pointer<Void>, int, Pointer<CFindView>)>("thinkthen_result_find");
  late final int Function(Pointer<Void>, int, Pointer<CAnnotateView>)
      thinkthen_result_annotate = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Size, Pointer<CAnnotateView>),
          int Function(Pointer<Void>, int,
              Pointer<CAnnotateView>)>("thinkthen_result_annotate");
  late final int Function(Pointer<Void>, int, Pointer<CRecognizeView>)
      thinkthen_result_recognize = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Size, Pointer<CRecognizeView>),
          int Function(Pointer<Void>, int,
              Pointer<CRecognizeView>)>("thinkthen_result_recognize");
  late final int Function(Pointer<Void>, int, Pointer<CRelateView>)
      thinkthen_result_relate = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Size, Pointer<CRelateView>),
          int Function(Pointer<Void>, int,
              Pointer<CRelateView>)>("thinkthen_result_relate");
  late final int Function(Pointer<Void>, Pointer<Pointer<Void>>)
      thinkthen_error_complete = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Pointer<Void>>),
          int Function(Pointer<Void>,
              Pointer<Pointer<Void>>)>("thinkthen_error_complete");
  late final int Function(
          Pointer<Void>, Pointer<CQuestionSpecView>, Pointer<Pointer<Void>>)
      thinkthen_question_new = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<CQuestionSpecView>,
              Pointer<Pointer<Void>>),
          int Function(Pointer<Void>, Pointer<CQuestionSpecView>,
              Pointer<Pointer<Void>>)>("thinkthen_question_new");
  late final int Function(Pointer<Void>, CStringView, Pointer<Pointer<Void>>)
      thinkthen_question_load = lib.lookupFunction<
          Int32 Function(Pointer<Void>, CStringView, Pointer<Pointer<Void>>),
          int Function(Pointer<Void>, CStringView,
              Pointer<Pointer<Void>>)>("thinkthen_question_load");
  late final int Function(
          Pointer<Void>,
          Pointer<CQuestionSpecView>,
          Pointer<CQuestionAuthorView>,
          Pointer<CRecognitionTaskView>,
          Pointer<Pointer<Void>>) thinkthen_question_new_recognition_v1 =
      lib.lookupFunction<
          Int32 Function(
              Pointer<Void>,
              Pointer<CQuestionSpecView>,
              Pointer<CQuestionAuthorView>,
              Pointer<CRecognitionTaskView>,
              Pointer<Pointer<Void>>),
          int Function(
              Pointer<Void>,
              Pointer<CQuestionSpecView>,
              Pointer<CQuestionAuthorView>,
              Pointer<CRecognitionTaskView>,
              Pointer<Pointer<Void>>)>("thinkthen_question_new_recognition_v1");
  late final int Function(Pointer<Void>, int, Pointer<CRecognitionTaskView>)
      thinkthen_result_recognition_task_v1 = lib
          .lookupFunction<
                  Int32 Function(
                      Pointer<Void>, Size, Pointer<CRecognitionTaskView>),
                  int Function(
                      Pointer<Void>, int, Pointer<CRecognitionTaskView>)>(
              "thinkthen_result_recognition_task_v1");
  late final int Function(Pointer<Void>, Pointer<CQuestionSpecView>,
          Pointer<CQuestionAuthorView>, Pointer<Pointer<Void>>)
      thinkthen_question_new_authored = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<CQuestionSpecView>,
              Pointer<CQuestionAuthorView>, Pointer<Pointer<Void>>),
          int Function(
              Pointer<Void>,
              Pointer<CQuestionSpecView>,
              Pointer<CQuestionAuthorView>,
              Pointer<Pointer<Void>>)>("thinkthen_question_new_authored");
  late final int Function(Pointer<Void>, Pointer<CQuestionAuthorView>)
      thinkthen_question_author = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<CQuestionAuthorView>),
          int Function(Pointer<Void>,
              Pointer<CQuestionAuthorView>)>("thinkthen_question_author");
  late final int Function(
          Pointer<Void>, int, CStringView, Pointer<Pointer<Void>>)
      thinkthen_question_parse = lib.lookupFunction<
          Int32 Function(
              Pointer<Void>, Uint32, CStringView, Pointer<Pointer<Void>>),
          int Function(Pointer<Void>, int, CStringView,
              Pointer<Pointer<Void>>)>("thinkthen_question_parse");
  late final int Function(
          Pointer<Void>, int, CStringView, Pointer<Pointer<Void>>)
      thinkthen_question_load_named = lib.lookupFunction<
          Int32 Function(
              Pointer<Void>, Uint32, CStringView, Pointer<Pointer<Void>>),
          int Function(Pointer<Void>, int, CStringView,
              Pointer<Pointer<Void>>)>("thinkthen_question_load_named");
  late final int Function(
          Pointer<Void>, int, CStringView, Pointer<Pointer<Void>>)
      thinkthen_question_load_reference = lib.lookupFunction<
          Int32 Function(
              Pointer<Void>, Uint32, CStringView, Pointer<Pointer<Void>>),
          int Function(Pointer<Void>, int, CStringView,
              Pointer<Pointer<Void>>)>("thinkthen_question_load_reference");
  late final int Function(Pointer<Void>, int, Pointer<CQuestionAuthorView>)
      thinkthen_result_question_author = lib.lookupFunction<
              Int32 Function(Pointer<Void>, Size, Pointer<CQuestionAuthorView>),
              int Function(Pointer<Void>, int, Pointer<CQuestionAuthorView>)>(
          "thinkthen_result_question_author");
  late final int Function(Pointer<Void>, int, int, Pointer<CQuestionAuthorView>)
      thinkthen_result_member_author = lib.lookupFunction<
          Int32 Function(
              Pointer<Void>, Size, Size, Pointer<CQuestionAuthorView>),
          int Function(Pointer<Void>, int, int,
              Pointer<CQuestionAuthorView>)>("thinkthen_result_member_author");
  late final int Function(Pointer<Void>, int, Pointer<CQuestionAuthorView>)
      thinkthen_result_observation_author = lib.lookupFunction<
              Int32 Function(Pointer<Void>, Size, Pointer<CQuestionAuthorView>),
              int Function(Pointer<Void>, int, Pointer<CQuestionAuthorView>)>(
          "thinkthen_result_observation_author");
  late final void Function(Pointer<Void>) thinkthen_question_free =
      lib.lookupFunction<Void Function(Pointer<Void>),
          void Function(Pointer<Void>)>("thinkthen_question_free");
  late final int Function(Pointer<Void>, Pointer<Uint8>, int, int,
          COptionalStringView, Pointer<Pointer<Void>>) thinkthen_image_clone =
      lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Uint8>, Size, Uint32,
              COptionalStringView, Pointer<Pointer<Void>>),
          int Function(
              Pointer<Void>,
              Pointer<Uint8>,
              int,
              int,
              COptionalStringView,
              Pointer<Pointer<Void>>)>("thinkthen_image_clone");
  late final int Function(Pointer<Void>, Pointer<CImageView>)
      thinkthen_image_view = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<CImageView>),
          int Function(
              Pointer<Void>, Pointer<CImageView>)>("thinkthen_image_view");
  late final void Function(Pointer<Void>) thinkthen_image_free =
      lib.lookupFunction<Void Function(Pointer<Void>),
          void Function(Pointer<Void>)>("thinkthen_image_free");
  late final int Function(
          Pointer<Void>, Pointer<CRecordView>, int, Pointer<Pointer<Void>>)
      thinkthen_source_records = lib.lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<CRecordView>, Size,
              Pointer<Pointer<Void>>),
          int Function(Pointer<Void>, Pointer<CRecordView>, int,
              Pointer<Pointer<Void>>)>("thinkthen_source_records");
  late final int Function(
          Pointer<Void>, Pointer<CSourceSpecView>, Pointer<Pointer<Void>>)
      thinkthen_source_files = lib.lookupFunction<
          Int32 Function(
              Pointer<Void>, Pointer<CSourceSpecView>, Pointer<Pointer<Void>>),
          int Function(Pointer<Void>, Pointer<CSourceSpecView>,
              Pointer<Pointer<Void>>)>("thinkthen_source_files");
  late final int Function(
          Pointer<Void>, Pointer<CSourceSpecView>, Pointer<Pointer<Void>>)
      thinkthen_source_image_files = lib.lookupFunction<
          Int32 Function(
              Pointer<Void>, Pointer<CSourceSpecView>, Pointer<Pointer<Void>>),
          int Function(Pointer<Void>, Pointer<CSourceSpecView>,
              Pointer<Pointer<Void>>)>("thinkthen_source_image_files");
  late final void Function(Pointer<Void>) thinkthen_source_free =
      lib.lookupFunction<Void Function(Pointer<Void>),
          void Function(Pointer<Void>)>("thinkthen_source_free");
}
