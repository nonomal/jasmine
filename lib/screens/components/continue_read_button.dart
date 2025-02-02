import 'package:flutter/material.dart';
import 'package:jasmine/basic/entities.dart';
import 'package:jasmine/basic/methods.dart';
import 'package:jasmine/screens/components/my_flat_button.dart';

// 继续阅读按钮
class ContinueReadButton extends StatefulWidget {
  final Future<ViewLog?> viewFuture;
  final AlbumResponse album;
  final Function(int epOrder, int pictureRank) onChoose;

  const ContinueReadButton({
    Key? key,
    required this.album,
    required this.onChoose,
    required this.viewFuture,
  }) : super(key: key);

  @override
  State<StatefulWidget> createState() => _ContinueReadButtonState();
}

class _ContinueReadButtonState extends State<ContinueReadButton> {
  @override
  Widget build(BuildContext context) {
    return FutureBuilder(
      future: widget.viewFuture,
      builder: (BuildContext context, AsyncSnapshot<ViewLog?> snapshot) {
        late void Function() onPressed;
        late String text;
        if (snapshot.connectionState != ConnectionState.done) {
          onPressed = () {};
          text = '加载中';
        } else {
          ViewLog? viewLog = snapshot.data;
          if (viewLog == null || viewLog.lastViewChapterId == 0) {
            if (widget.album.series.isEmpty) {
              return Container();
            }
            onPressed = () {
              if (widget.album.series.isEmpty) {
                widget.onChoose(widget.album.id, 0);
              } else {
                widget.album.series.sort(
                  (a, b) => int.parse(a.sort).compareTo(int.parse(b.sort)),
                );
                widget.onChoose(widget.album.series[0].id, 0);
              }
            };
            text = '从头开始';
          } else {
            onPressed = () {
              widget.onChoose(
                viewLog.lastViewChapterId,
                viewLog.lastViewPage,
              );
            };
            text = '继续阅读'; // todo names and pages
            if (widget.album.series.isNotEmpty) {
              for (var i = 0; i < widget.album.series.length; i++) {
                if (widget.album.series[i].id == viewLog.lastViewChapterId) {
                  text += " (${i + 1}.";
                  if (widget.album.series[i].name.isNotEmpty) {
                    text += widget.album.series[i].name + ".";
                  }
                  text += "P${viewLog.lastViewPage + 1})";
                  break;
                }
              }
            } else {
              text += " (P${viewLog.lastViewPage + 1})";
            }
          }
        }
        return MyFlatButton(title: text, onPressed: onPressed);
      },
    );
  }
}
